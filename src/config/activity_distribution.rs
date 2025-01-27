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