use std::error::{self, Error};

use octocrab::params::repos::forks::Sort;

const GIT_OWNER: &str = "spamserv";
const GIT_REPO: &str = "gitspam";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>>{
    // Get repo
    let repo = octocrab::instance()
        .repos(GIT_OWNER, GIT_REPO)
        .list_contributors()
        .send()
        .await?;

    Ok(())

    //format!("{:?}", repo);
    // Get random configuration based on inputs

    // Create commits
    // Create pull requests
    // Create issues
    // Create code reviews
}
