use super::config::Config;
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
    pub fn new(config: &Config, activities: u16) -> Self {
        Self::calculate_activity_matrix_from_distribuion(config, activities)
    }

    fn calculate_activity_matrix_from_distribuion(config: &Config, activities: u16) -> ActivityDistributionMatrix{
        let mut adm = ActivityDistributionMatrix {
            commits: (config.commits * activities as f64).round(),
            pull_requests: (config.pull_requests * activities as f64).round(),
            code_reviews: (config.code_reviews * activities as f64).round(),
            issues: (config.issues * activities as f64).round(),
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

        adm
    }
}