use serde::Deserialize;

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