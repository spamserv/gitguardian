use dotenvy::dotenv;
use octocrab::models::repos::CommitAuthor;
use rand::Rng;
use std::env;

const GIT_OWNER: &str = "spamserv";
const GIT_REPO: &str = "gitspam";
const GITHUB_PERSONAL_ACCESS_TOKEN: &str = "GITHUB_PERSONAL_ACCESS_TOKEN";

const GITHUB_NAME: &str = "Josip Vojak";
const GITHUB_EMAIL: &str = "josipvojak@gmail.com";
const 

#[derive(Debug)]
struct ActivityDistributionMatrix {
    commits: f64,
    pull_requests: f64,
    code_reviews: f64,
    issues: f64,
    daily_activities: DailyActivity 
}

#[derive(Debug, Clone)]
struct DailyActivity {
    low: u16,
    high: u16
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    let daily_activities = DailyActivity {
        low: 2,
        high: 27
    };

    let mut rng = rand::thread_rng();
    let num_tasks = rng.gen_range(daily_activities.low..=daily_activities.high);
    println!("Total tasks for the day: {}", num_tasks);

    let activity_distribution_matrix = ActivityDistributionMatrix {
        commits: 0.55,
        pull_requests: 0.08,
        code_reviews: 0.20,
        issues: 0.17,
        daily_activities: daily_activities.clone()
    };

    let mut total_activity_distribution = ActivityDistributionMatrix {
        commits: (activity_distribution_matrix.commits * num_tasks as f64).round(),
        pull_requests: (activity_distribution_matrix.pull_requests * num_tasks as f64).round(),
        code_reviews: (activity_distribution_matrix.code_reviews * num_tasks as f64).round(),
        issues: (activity_distribution_matrix.issues * num_tasks as f64).round(),
        daily_activities: daily_activities.clone()
    };

    let total_calculated= (total_activity_distribution.commits + total_activity_distribution.pull_requests + total_activity_distribution.issues + total_activity_distribution.code_reviews) as u16;

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

    let page = octocrab
        .current()
        .list_repos_for_authenticated_user()
        .visibility("private")
        .send()
        .await?;

    let repo = octocrab.repos(GIT_OWNER, GIT_REPO).list_branches().send().await?;



    // Create commits
    octocrab.repos(GIT_OWNER, GIT_REPO)
        .create_git_commit_object("This is my message", "tree")
        .signature("My Signature")
        .author(CommitAuthor {name:GITHUB_NAME.to_owned(),email:GITHUB_EMAIL.to_owned(), date: todo!() })
        .committer("What is a commiter>")
        .send().await?;
    // Create pull requests
    // Create issues
    // Create code reviews
    Ok(())
}
