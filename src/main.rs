use dotenvy::dotenv;
use gitguardian::{
    config::config::Config,
    git_manager::manager::GitManager,
};

const CONFIG_FILE: &str = "config.toml";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load env variables
    dotenv().ok();

    // Read config
    let config = Config::read_from_string(CONFIG_FILE)?;

    // Create a matrix of activities
    let adm = config.get_activity_distribution_matrix();

    let git_manager = GitManager::new(adm).await?;
    git_manager.enable_branch_autodelete().await?;
    git_manager.create_commits().await?;
    git_manager.create_issues().await?;
    git_manager.create_pull_requests().await?;

    Ok(())
}
