use base64::engine::general_purpose;
use base64::Engine;
use dotenvy::dotenv;
use gitguardian::{config::activity_distribution::{self, ActivityDistributionMatrix, DailyActivity}, constants::github, git_models::git_models::{CreateRef, CreateReviewRequest, UpdateFileRequest, UpdateFileResponse, UpdateRepo}};
use rand::Rng;
use serde::{Deserialize, Serialize};
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    let daily_activities = DailyActivity { low: 2, high: 13 };

    let mut rng = rand::thread_rng();
    let activities = rng.gen_range(daily_activities.low..=daily_activities.high);
    println!("Total tasks for the day: {}", activities);

    let activity_distribution_matrix = ActivityDistributionMatrix::new(0.55, 0.08, 0.20, 0.17, activities);
   

    println!("{:?}", activity_distribution_matrix);

    let octocrab = octocrab::Octocrab::builder()
        .personal_token(
            env::var(github::GITHUB_PERSONAL_ACCESS_TOKEN)
                .to_owned()
                .expect("Missing GITHUB_PERSONAL_ACCESS_TOKEN."),
        )
        .build()?;

    // 0. Update repo settings to enable auto-delete of a merged branch
    let repo_url = format!("/repos/{}/{}", github::GIT_OWNER, github::GIT_REPO);
    let update_settings = UpdateRepo {
        delete_branch_on_merge: true,
    };
    let _updated_repo: serde_json::Value = octocrab.patch(repo_url, Some(&update_settings)).await?;

    // println!("Repository settings updated: {:#}", updated_repo);
    let readme_url = format!("/repos/{}/{}/contents/{}", github::GIT_OWNER, github::GIT_REPO, github::README_FILE_PATH);
    for commit_index in 0..activity_distribution_matrix.commits as u16 {
        // 1. Create commits (README.md must exist and it already exist...but otherwise check and create if it does not)
        let readme_content = octocrab
            .repos(github::GIT_OWNER, github::GIT_REPO)
            .get_content()
            .path(github::README_FILE_PATH)
            .send()
            .await?;

        //println!("{:?}", readme_content);
        let file_data = readme_content.items.get(0);
        let sha = readme_content.items.get(0).map(|i| i.sha.as_str());
        // let url = file_data.ok_or("File does not exist")?.url.as_str();
        // println!("{}", url);
        //println!("{:?} {}", sha, url);

        let message = format!("docs: update README.md {}", commit_index);
        let update_body = UpdateFileRequest {
            message: &message,
            content: general_purpose::STANDARD.encode("Updated README content!"),
            sha,                         // if Some(...) => update; if None => create
            branch: Some(github::GITHUB_BRANCH), // change to "master" or other if needed
        };

        octocrab
            .put::<UpdateFileResponse, _, _>(readme_url.clone(), Some(&update_body))
            .await?;
    }

    for issue_index in 0..activity_distribution_matrix.issues as u16 {
        // 2. Create issues
        let title = format!("{} #{}", github::GITHUB_ISSUE_NAME, issue_index);
        octocrab
            .issues(github::GIT_OWNER, github::GIT_REPO)
            .create(title)
            .body(github::GITHUB_ISSUE_BODY)
            .send()
            .await?;
    }

    /*
        3. Create pull requests:
            0. Get base branch SHA
            1. Create branch
            2. Push commit
            3. Create pull request
            4. Do code review
            5. Merge pull request
            6. Delete branch
    */

    for pull_request_index in 0..activity_distribution_matrix.issues as u16 {
        let readme_content = octocrab
            .repos(github::GIT_OWNER, github::GIT_REPO)
            .get_content()
            .path(github::README_FILE_PATH)
            .send()
            .await?;
        let offset = chrono::offset::Local::now().timestamp_micros().to_string();

        let sha = readme_content.items.get(0).map(|i| i.sha.as_str());

        // 0. Get base branch SHA
        let get_ref_url = format!("/repos/{}/{}/git/ref/heads/{}", github::GIT_OWNER, github::GIT_REPO, github::GITHUB_BRANCH);
        let base_ref_value: serde_json::Value = octocrab.get(get_ref_url, None::<&()>).await?;
        let base_sha = base_ref_value["object"]["sha"]
            .as_str()
            .ok_or("Could not extract base branch SHA")?
            .to_string();

        // 1. Create branch
        let pull_request_branch_name = format!("{}-{}", github::GITHUB_PULL_REQUEST_BRANCH, offset);
        let ref_ = format!("refs/heads/{}", pull_request_branch_name);
        let create_ref_body = CreateRef {
            ref_,
            sha: base_sha.clone(),
        };

        let post_ref_url = format!("/repos/{}/{}/git/refs", github::GIT_OWNER, github::GIT_REPO);
        let _create_branch_response: serde_json::Value = octocrab
            .post::<CreateRef, _>(post_ref_url, Some(&create_ref_body))
            .await?;
        //println!("Create branch response: {:#}", create_branch_response);

        // Make a new commit
        let message = format!("{} - New branch PR: update README.md", pull_request_index);
        let update_body = UpdateFileRequest {
            message: &message,
            content: general_purpose::STANDARD.encode("Pull Request: Updated README content!"),
            sha,                                      // if Some(...) => update; if None => create
            branch: Some(&pull_request_branch_name), // change to "master" or other if needed
        };

        // 2. Push Commit
        octocrab
            .put::<UpdateFileResponse, _, _>(readme_url.clone(), Some(&update_body))
            .await?;

        // 3. Create pull request
        let pull_request_title = format!(
            "{} - {}",
            chrono::offset::Local::now(),
            github::GITHUB_PULL_REQUEST_TITLE
        );
        let pull_request = octocrab
            .pulls(github::GIT_OWNER, github::GIT_REPO)
            .create(
                pull_request_title,
                pull_request_branch_name,
                github::GITHUB_BRANCH,
            )
            .body(github::GITHUB_PULL_REQUEST_BODY)
            .send()
            .await?;

        //println!("Pull request response {:?}", pull_request);

        // 4. Create review request
        let review_request = CreateReviewRequest {
            body: Some("Looks good to me!"),
            event: Some("COMMENT"), // or "APPROVE", "REQUEST_CHANGES" // cannot do self APPROVAL
            comments: Some(vec![
                // ReviewComment {
                //     path: README_FILE_PATH,
                //     position: 0,  // line index in the diff (not the file line number)
                //     body: "This should create a comment on the line number 0.",
                // },
            ]), // no inline comments in this example
        };
        let review_request_url = format!(
            "/repos/{}/{}/pulls/{}/reviews",
            github::GIT_OWNER, github::GIT_REPO, pull_request.number
        );
        let _review_request_response: serde_json::Value = octocrab
            .post::<CreateReviewRequest, _>(review_request_url, Some(&review_request))
            .await?;

        // 5. Merge pull request
        let merge_response = octocrab
            .pulls(github::GIT_OWNER, github::GIT_REPO)
            .merge(pull_request.number)
            .send()
            .await?;
        println!("{:?}", merge_response);
    }

    Ok(())
}
