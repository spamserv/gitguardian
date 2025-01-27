use base64::engine::general_purpose;
use base64::Engine;
use chrono::offset::Utc;
use dotenvy::dotenv;
use octocrab::models::repos::CommitAuthor;
use octocrab::Octocrab;
use rand::Rng;
use serde::Deserialize;
use std::env;

const GIT_OWNER: &str = "spamserv";
const GIT_REPO: &str = "gitspam";
const GITHUB_PERSONAL_ACCESS_TOKEN: &str = "GITHUB_PERSONAL_ACCESS_TOKEN";

const GITHUB_NAME: &str = "Josip Vojak";
const GITHUB_EMAIL: &str = "josipvojak@gmail.com";
const GITHUB_BRANCH: &str = "main";

const README_FILE_PATH: &str = "README.md";
const README_FILE_CONTENT: &str = "This is a test message.";
const GITHUB_COMMIT_MESSAGE: &str = "This is a test commit using octocrab";

const GITHUB_ISSUE_NAME: &str = "Issue#1";
const GITHUB_ISSUE_BODY: &str = "Issue#1 Body";

const GITHUB_PULL_REQUEST_BRANCH: &str = "pull-request-branch";
const GITHUB_PULL_REQUEST_TITLE: &str = "Let's test this pull request.";
const GITHUB_PULL_REQUEST_BODY: &str = "This is a body of pull request.";
#[derive(Debug)]
struct ActivityDistributionMatrix {
    commits: f64,
    pull_requests: f64,
    code_reviews: f64,
    issues: f64,
    daily_activities: DailyActivity,
}

#[derive(Debug, Clone)]
struct DailyActivity {
    low: u16,
    high: u16,
}

#[derive(serde::Serialize)]
struct UpdateFileRequest<'a> {
    message: &'a str,
    content: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    sha: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    branch: Option<&'a str>,
}

#[derive(Debug, Deserialize)]
struct UpdateFileResponse {
    commit: CommitInfo,
    content: Option<FileContent>,
}

#[derive(Debug, Deserialize)]
struct CommitInfo {
    sha: String,
    // plus any other fields you might need
}

#[derive(Debug, Deserialize)]
struct FileContent {
    sha: String,
    // plus any other fields you might need
}

#[derive(serde::Serialize)]
struct CreateRef {
    // GitHub expects the full ref format, e.g. "refs/heads/my-new-branch"
    #[serde(rename = "ref")]
    ref_: String,
    sha: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    let daily_activities = DailyActivity { low: 2, high: 27 };

    let mut rng = rand::thread_rng();
    let num_tasks = rng.gen_range(daily_activities.low..=daily_activities.high);
    println!("Total tasks for the day: {}", num_tasks);

    let activity_distribution_matrix = ActivityDistributionMatrix {
        commits: 0.55,
        pull_requests: 0.08,
        code_reviews: 0.20,
        issues: 0.17,
        daily_activities: daily_activities.clone(),
    };

    let mut total_activity_distribution = ActivityDistributionMatrix {
        commits: (activity_distribution_matrix.commits * num_tasks as f64).round(),
        pull_requests: (activity_distribution_matrix.pull_requests * num_tasks as f64).round(),
        code_reviews: (activity_distribution_matrix.code_reviews * num_tasks as f64).round(),
        issues: (activity_distribution_matrix.issues * num_tasks as f64).round(),
        daily_activities: daily_activities.clone(),
    };

    let total_calculated = (total_activity_distribution.commits
        + total_activity_distribution.pull_requests
        + total_activity_distribution.issues
        + total_activity_distribution.code_reviews) as u16;

    let diff: i16 = num_tasks as i16 - total_calculated as i16;
    if diff != 0 {
        total_activity_distribution.commits += diff as f64
    }

    println!("{:?}", total_activity_distribution);

    let octocrab = octocrab::Octocrab::builder()
        .personal_token(
            env::var(GITHUB_PERSONAL_ACCESS_TOKEN)
                .to_owned()
                .expect("Missing GITHUB_PERSONAL_ACCESS_TOKEN."),
        )
        .build()?;

    // Create commits (README.md must exist and it already exist...but otherwise check and create if it does not)
    let readme_content = octocrab
        .repos(GIT_OWNER, GIT_REPO)
        .get_content()
        .path(README_FILE_PATH)
        .send()
        .await?;

    //println!("{:?}", readme_content);
    let file_data = readme_content.items.get(0);
    let sha = readme_content.items.get(0).map(|i| i.sha.as_str());
    let url = file_data.ok_or("File does not exist")?.url.as_str();

    //println!("{:?} {}", sha, url);

    // let commit_response = octocrab
    //     .repos(GIT_OWNER, GIT_REPO)
    //     .create_file(README_FILE_PATH, GITHUB_COMMIT_MESSAGE, README_FILE_CONTENT)
    //     .branch(GITHUB_BRANCH) // or whatever branch you want
    //     .author(CommitAuthor {
    //         name: GITHUB_NAME.to_owned(),
    //         email: GITHUB_EMAIL.to_owned(),
    //         date: Some(Utc::now()),
    //     })
    //     .send()
    //     .await?;

    let update_body = UpdateFileRequest {
        message: "docs: update README.md",
        content: general_purpose::STANDARD.encode("Updated README content!"),
        sha,                         // if Some(...) => update; if None => create
        branch: Some(GITHUB_BRANCH), // change to "master" or other if needed
    };

    // octocrab
    //     .put::<UpdateFileResponse, _, _>(url, Some(&update_body))
    //     .await?;

    // Create issues
    // octocrab
    //     .issues(GIT_OWNER, GIT_REPO)
    //     .create(GITHUB_ISSUE_NAME)
    //     .body(GITHUB_ISSUE_BODY)
    //     .send()
    //     .await?;

    /*
        Create pull requests:
        0. Get base branch SHA
        1. Create branch
        2. Push commit
        3. Create pull request
    */

    // 0. Get base branch SHA
    let get_ref_url = format!("/repos/{GIT_OWNER}/{GIT_REPO}/git/ref/heads/{GITHUB_BRANCH}");
    let base_ref_value: serde_json::Value = octocrab.get(get_ref_url, None::<&()>).await?;
    let base_sha = base_ref_value["object"]["sha"]
        .as_str()
        .ok_or("Could not extract base branch SHA")?
        .to_string();

    // 1. Create branch
    let ref_ = format!("refs/heads/{}", GITHUB_PULL_REQUEST_BRANCH);
    let create_ref_body = CreateRef {
        ref_,
        sha: base_sha,
    };

    let post_ref_url = format!("/repos/{GIT_OWNER}/{GIT_REPO}/git/refs");
    let create_branch_response: serde_json::Value = octocrab
        .post::<CreateRef, _>(post_ref_url, Some(&create_ref_body))
        .await?;
    println!("Create branch response: {:#}", create_branch_response);
    
    // Make a new commit
    let update_body = UpdateFileRequest {
        message: "New branch PR: update README.md",
        content: general_purpose::STANDARD.encode("Pull Request: Updated README content!"),
        sha,                         // if Some(...) => update; if None => create
        branch: Some(GITHUB_PULL_REQUEST_BRANCH), // change to "master" or other if needed
    };

    octocrab
        .put::<UpdateFileResponse, _, _>(url, Some(&update_body))
        .await?;

    octocrab
        .pulls(GIT_OWNER, GIT_REPO)
        .create(
            GITHUB_PULL_REQUEST_TITLE,
            GITHUB_PULL_REQUEST_BRANCH,
            GITHUB_BRANCH,
        )
        .body(GITHUB_PULL_REQUEST_BODY)
        .send()
        .await?;

    // Create code reviews
    Ok(())
}
