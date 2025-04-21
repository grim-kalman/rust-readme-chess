use std::env;

#[derive(Clone, Debug)]
pub struct Config {
    pub github_url: String,
    pub github_api_url: String,
    pub engine_path: String,
    pub server_addr: String,
}

impl Config {
    /// Load configuration from environment variables.
    pub fn from_env() -> Result<Self, env::VarError> {
        Ok(Self {
            github_url: env::var("GITHUB_URL")?,
            github_api_url: env::var("GITHUB_API_URL")?,
            engine_path: env::var("ENGINE_PATH")?,
            server_addr: env::var("SERVER_ADDR")?,
        })
    }
}
