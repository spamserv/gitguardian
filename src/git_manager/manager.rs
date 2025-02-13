use std::env;

use base64::{engine::general_purpose, Engine};
use octocrab::{Error, Octocrab};

use crate::{
    config::activity_distribution::ActivityDistributionMatrix,
    constants::github,
    git_models::git_models::{
        CreateRef, CreateReviewRequest, UpdateFileRequest, UpdateFileResponse, UpdateRepo,
    },
};

pub struct GitManager {
    octocrab: Octocrab,
    activity_distribution: ActivityDistributionMatrix,
    readme_url: String,
}

impl GitManager {
    pub async fn new(activity: ActivityDistributionMatrix) -> Result<GitManager, Error> {
        let readme_url = format!(
            "/repos/{}/{}/contents/{}",
            github::GIT_OWNER,
            github::GIT_REPO,
            github::README_FILE_PATH
        );
        let octocrab = octocrab::Octocrab::builder()
            .personal_token(
                env::var(github::GITHUB_PERSONAL_ACCESS_TOKEN)
                    .to_owned()
                    .expect("Missing GITHUB_PERSONAL_ACCESS_TOKEN."),
            )
            .build()?;

        let manager = GitManager {
            readme_url,
            activity_distribution: activity,
            octocrab,
        };
        Ok(manager)
    }

    pub async fn enable_branch_autodelete(&self) -> Result<(), Error> {
        // 0. Update repo settings to enable auto-delete of a merged branch
        let repo_url = format!("/repos/{}/{}", github::GIT_OWNER, github::GIT_REPO);
        let update_settings = UpdateRepo {
            delete_branch_on_merge: true,
        };
        let _updated_repo: serde_json::Value = self
            .octocrab
            .patch(repo_url, Some(&update_settings))
            .await?;

        //println!("Repository settings updated: {:#}", _updated_repo);

        Ok(())
    }

    pub async fn create_commits(&self) -> Result<(), Box<dyn std::error::Error>> {
        for commit_index in 0..self.activity_distribution.commits as u16 {
            let readme_content = self
                .octocrab
                .repos(github::GIT_OWNER, github::GIT_REPO)
                .get_content()
                .path(github::README_FILE_PATH)
                .send()
                .await?;

            let sha = readme_content.items.first().map(|i| i.sha.as_str());
            let message = format!("docs: update README.md {}", commit_index);
            let content = format!("Updated README content with commit index #{}!", commit_index);
            let update_body = UpdateFileRequest {
                message: &message,
                content: general_purpose::STANDARD.encode(content),
                sha,                                 // if Some(...) => update; if None => create
                branch: Some(github::GITHUB_BRANCH), // change to "master" or other if needed
            };
            self.octocrab
                .put::<UpdateFileResponse, _, _>(self.readme_url.clone(), Some(&update_body))
                .await?;
        }
        Ok(())
    }

    pub async fn create_issues(&self) -> Result<(), Box<dyn std::error::Error>> {
        for issue_index in 0..self.activity_distribution.issues as u16 {
            // 2. Create issues
            let title = format!("{} #{}", github::GITHUB_ISSUE_NAME, issue_index);
            self.octocrab
                .issues(github::GIT_OWNER, github::GIT_REPO)
                .create(title)
                .body(github::GITHUB_ISSUE_BODY)
                .send()
                .await?;
        }
        Ok(())
    }

    async fn create_branch(
        &self,
        base_sha: &str,
        branch_name: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        // 1. Create branch
        let ref_ = format!("refs/heads/{}", branch_name);
        let create_ref_body = CreateRef {
            ref_,
            sha: base_sha.to_owned(),
        };

        let post_ref_url = format!("/repos/{}/{}/git/refs", github::GIT_OWNER, github::GIT_REPO);
        let _create_branch_response: serde_json::Value = self
            .octocrab
            .post::<CreateRef, _>(post_ref_url, Some(&create_ref_body))
            .await?;

        Ok(())
    }

    pub async fn create_pull_requests(&self) -> Result<(), Box<dyn std::error::Error>> {
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

        for pull_request_index in 0..self.activity_distribution.issues as u16 {
            let readme_content = self
                .octocrab
                .repos(github::GIT_OWNER, github::GIT_REPO)
                .get_content()
                .path(github::README_FILE_PATH)
                .send()
                .await?;
            let offset = chrono::offset::Local::now().timestamp_micros().to_string();

            let sha = readme_content.items.first().map(|i| i.sha.as_str());

            // 0. Get base branch SHA
            let get_ref_url = format!(
                "/repos/{}/{}/git/ref/heads/{}",
                github::GIT_OWNER,
                github::GIT_REPO,
                github::GITHUB_BRANCH
            );
            let base_ref_value: serde_json::Value =
                self.octocrab.get(get_ref_url, None::<&()>).await?;
            let base_sha = base_ref_value["object"]["sha"]
                .as_str()
                .ok_or("Could not extract base branch SHA")?
                .to_string();

            let pull_request_branch_name =
                format!("{}-{}", github::GITHUB_PULL_REQUEST_BRANCH, offset);

            self.create_branch(&base_sha, &pull_request_branch_name)
                .await?;
            //println!("Create branch response: {:#}", create_branch_response);

            // Make a new commit
            let message = format!("{} - New branch PR: update README.md", pull_request_index);
            let update_body = UpdateFileRequest {
                message: &message,
                content: general_purpose::STANDARD.encode("Pull Request: Updated README content!"),
                sha, // if Some(...) => update; if None => create
                branch: Some(&pull_request_branch_name), // change to "master" or other if needed
            };

            // 2. Push Commit
            self.octocrab
                .put::<UpdateFileResponse, _, _>(self.readme_url.clone(), Some(&update_body))
                .await?;

            // 3. Create pull request
            let pull_request_title = format!(
                "{} - {}",
                chrono::offset::Local::now(),
                github::GITHUB_PULL_REQUEST_TITLE
            );
            let pull_request = self
                .octocrab
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
                github::GIT_OWNER,
                github::GIT_REPO,
                pull_request.number
            );
            let _review_request_response: serde_json::Value = self
                .octocrab
                .post::<CreateReviewRequest, _>(review_request_url, Some(&review_request))
                .await?;

            // 5. Merge pull request
            let merge_response = self
                .octocrab
                .pulls(github::GIT_OWNER, github::GIT_REPO)
                .merge(pull_request.number)
                .send()
                .await?;
            println!("{:?}", merge_response);
        }
        Ok(())
    }
}
