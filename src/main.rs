use dotenvy::dotenv;
use gitguardian::{
    config::{activity_distribution::ActivityDistributionMatrix, config::Data},
    git_manager::manager::GitManager,
};
use rand::Rng;

const CONFIG_FILE: &str = "config.toml";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    let config = std::fs::read_to_string(CONFIG_FILE)?;
    let config_data: Data = toml::from_str(&config)?;

    let mut rng = rand::thread_rng();
    let activities = rng.gen_range(config_data.config.low..=config_data.config.high);
    println!("Total tasks for the day: {}", activities);

    let activity_distribution_matrix = ActivityDistributionMatrix::new(config_data, activities);

    println!("{:?}", activity_distribution_matrix);
    let git_manager = GitManager::new(activity_distribution_matrix).await?;
    git_manager.enable_branch_autodelete().await?;
    git_manager.create_commits().await?;
    git_manager.create_issues().await?;
    git_manager.create_pull_requests().await?;

    Ok(())
}
