use rand::Rng;
use serde::Deserialize;

use super::activity_distribution::ActivityDistributionMatrix;

// Top level struct to hold the TOML data.
#[derive(Deserialize)]
pub struct Data {
    pub config: Config,
}

// Config struct holds to data from the `[config]` section.
#[derive(Deserialize)]
pub struct Config {
    pub commits: f64,
    pub pull_requests: f64,
    pub code_reviews: f64,
    pub issues: f64,
    pub low: u16,
    pub high: u16,
}

impl Config {
    pub fn read_from_string(file: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let config = std::fs::read_to_string(file)?;
        let config_data: Data = toml::from_str(&config)?;
        Ok(config_data.config)
    }

    fn calculate_activities(&self) -> u16 {
        let mut rng = rand::thread_rng();
        let activities = rng.gen_range(self.low..=self.high);
        println!("Total tasks for the day: {}", activities);

        activities
    }

    pub fn get_activity_distribution_matrix(&self) -> ActivityDistributionMatrix {
        let activities = self.calculate_activities();
        let adm = ActivityDistributionMatrix::new(self, activities);
        println!("{:?}", adm);
        adm
    }
}