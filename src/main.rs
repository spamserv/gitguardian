use base64::engine::general_purpose;
use base64::Engine;
use dotenvy::dotenv;
use gitguardian::{config::activity_distribution::{self, ActivityDistributionMatrix, DailyActivity}, constants::github, git_manager::manager::GitManager, git_models::git_models::{CreateRef, CreateReviewRequest, UpdateFileRequest, UpdateFileResponse, UpdateRepo}};
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
    let git_manager = GitManager::new(activity_distribution_matrix).await?;
    git_manager.enable_branch_autodelete().await?;
    git_manager.create_commits().await?;
    git_manager.create_issues().await?;
    git_manager.create_pull_requests().await?;

    Ok(())
}
