use std::env;

use octocrab::{Error, Octocrab};
use serde::de::value::Error;

use crate::{config::activity_distribution::ActivityDistributionMatrix, constants::github, git_models::git_models::UpdateRepo};

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

    fn create_commit() {}

    fn create_issue() {}

    fn create_branch() {}

    fn create_pull_request() {}
}
