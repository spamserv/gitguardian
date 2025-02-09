#[derive(Debug)]
pub struct ActivityDistributionMatrix {
    pub commits: f64,
    pub pull_requests: f64,
    pub code_reviews: f64,
    pub issues: f64,
    pub daily_activities: DailyActivity,
}

#[derive(Debug, Clone)]
pub struct DailyActivity {
    pub low: u16,
    pub high: u16,
}