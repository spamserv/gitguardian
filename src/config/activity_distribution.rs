#[derive(Debug)]
pub struct ActivityDistributionMatrix {
    pub commits: f64,
    pub pull_requests: f64,
    pub code_reviews: f64,
    pub issues: f64,
    pub activities: u16,
}

#[derive(Debug, Clone)]
pub struct DailyActivity {
    pub low: u16,
    pub high: u16,
}

impl ActivityDistributionMatrix {
    pub fn new(commits: f64, pull_requests: f64, code_reviews: f64, issues: f64, activities: u16) -> Self {

        let activity_matrix = Self::calculate_activity_matrix_from_distribuion(commits, pull_requests, code_reviews, issues, activities);
        activity_matrix
    }

    fn calculate_activity_matrix_from_distribuion(commits: f64, pull_requests: f64, code_reviews: f64, issues: f64, activities: u16) -> ActivityDistributionMatrix{
        let mut adm = ActivityDistributionMatrix {
            commits: (commits * activities as f64).round(),
            pull_requests: (pull_requests * activities as f64).round(),
            code_reviews: (code_reviews * activities as f64).round(),
            issues: (issues * activities as f64).round(),
            activities,
        };
    
        let total_calculated = (adm.commits
            + adm.pull_requests
            + adm.issues
            + adm.code_reviews) as u16;
    
        let diff: i16 = activities as i16 - total_calculated as i16;
        if diff != 0 {
            adm.commits += diff as f64;
        }

        return adm
    }
}