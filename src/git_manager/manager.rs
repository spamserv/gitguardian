use std::env;

use base64::{engine::general_purpose, Engine};
use octocrab::{Error, Octocrab};
use serde::de::value::Error;

use crate::{config::activity_distribution::ActivityDistributionMatrix, constants::github, git_models::git_models::{UpdateFileRequest, UpdateFileResponse, UpdateRepo}};

pub struct GitManager {
    octocrab: Octocrab,
    activity_distribution: ActivityDistributionMatrix,
}

impl GitManager {
    pub async fn new(activity: ActivityDistributionMatrix) -> Result<GitManager, Error> {
        let octocrab = octocrab::Octocrab::builder()
            .personal_token(
                env::var(github::GITHUB_PERSONAL_ACCESS_TOKEN)
                    .to_owned()
                    .expect("Missing GITHUB_PERSONAL_ACCESS_TOKEN."),
            )
            .build()?;
        
        let manager = GitManager {
            activity_distribution: activity,
            octocrab
        }
        Ok(manager)
    }

    pub async fn enable_branch_autodelete(&self) -> Result<(), Error> {

        // 0. Update repo settings to enable auto-delete of a merged branch
        let repo_url = format!("/repos/{}/{}", github::GIT_OWNER, github::GIT_REPO);
        let update_settings = UpdateRepo {
            delete_branch_on_merge: true,
        };
        let _updated_repo: serde_json::Value = self.octocrab.patch(repo_url, Some(&update_settings)).await?;

        Ok(())
    }

    pub async fn create_commits(&self) -> Result<(),  Box<dyn std::error::Error>> {
        // println!("Repository settings updated: {:#}", updated_repo);
        let readme_url = format!("/repos/{}/{}/contents/{}", github::GIT_OWNER, github::GIT_REPO, github::README_FILE_PATH);
        for commit_index in 0..self.activity_distribution.commits as u16 {
            let readme_content = self.octocrab
                    .repos(github::GIT_OWNER, github::GIT_REPO)
                    .get_content()
                    .path(github::README_FILE_PATH)
                    .send()
                    .await?;
            let file_data = readme_content.items.get(0);
            let sha = readme_content.items.get(0).map(|i| i.sha.as_str());
            let message = format!("docs: update README.md {}", commit_index);
            let update_body = UpdateFileRequest {
                    message: &message,
                    content: general_purpose::STANDARD.encode("Updated README content!"),
                    sha,                         // if Some(...) => update; if None => create
                    branch: Some(github::GITHUB_BRANCH), // change to "master" or other if needed
                };
            self.octocrab
                    .put::<UpdateFileResponse, _, _>(readme_url.clone(), Some(&update_body))
                    .await?;
        }
        Ok(())
    }

    pub async fn create_issues(&self) -> Result<(), Box<dyn std::error::Error>>{
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

    fn create_branch() {}

    fn create_pull_request() {}
}
