use reqwest::{Client, Method};
use serde_json::{Value, json};
use std::sync::Arc;
use base64::engine::general_purpose::STANDARD as BASE64_STANDARD;
use base64::Engine as _;

/// Configuration for GitHub API operations.
pub struct GithubConfig {
    pub token: String,
    pub api_url: String,
    pub owner_repo: String,
    pub readme_path: String,
    pub branch: String,
}

#[derive(Clone)]
/// Service for updating the GitHub README file.
pub struct GithubService {
    client: Client,
    config: Arc<GithubConfig>,
}

impl GithubService {
    /// Create a new GithubService with the given config.
    pub fn new(config: Arc<GithubConfig>) -> Self {
        Self {
            client: Client::new(),
            config,
        }
    }

    /// Update the README file on GitHub with the new board markdown.
    pub async fn update_readme(&self, board_markdown: &str) -> anyhow::Result<()> {
        let latest_commit_sha = self.get_latest_commit_sha().await?;
        let new_tree_sha = self.create_tree_sha(&latest_commit_sha, board_markdown).await?;
        let new_commit_sha = self.create_commit_sha(&latest_commit_sha, &new_tree_sha).await?;
        self.update_ref_with_new_commit(&new_commit_sha).await?;
        Ok(())
    }

    /// Get the latest commit SHA for the branch.
    async fn get_latest_commit_sha(&self) -> anyhow::Result<String> {
        let endpoint = format!("git/refs/heads/{}", self.config.branch);
        let resp = self.handle_request(&endpoint, Method::GET, None).await?;
        Ok(resp["object"]["sha"].as_str().unwrap().to_string())
    }

    /// Create a new tree SHA with the updated README content.
    async fn create_tree_sha(
        &self,
        latest_commit_sha: &str,
        new_board_state: &str,
    ) -> anyhow::Result<String> {
        let json = json!({
            "base_tree": latest_commit_sha,
            "tree": [{
                "path": self.config.readme_path,
                "mode": "100644",
                "type": "blob",
                "content": new_board_state
            }]
        });
        let resp = self.handle_request("git/trees", Method::POST, Some(json)).await?;
        Ok(resp["sha"].as_str().unwrap().to_string())
    }

    /// Create a new commit SHA for the updated tree.
    async fn create_commit_sha(
        &self,
        latest_commit_sha: &str,
        new_tree_sha: &str,
    ) -> anyhow::Result<String> {
        let json = json!({
            "message": "Update README",
            "parents": [latest_commit_sha],
            "tree": new_tree_sha
        });
        let resp = self.handle_request("git/commits", Method::POST, Some(json)).await?;
        Ok(resp["sha"].as_str().unwrap().to_string())
    }

    /// Update the branch ref to point to the new commit.
    async fn update_ref_with_new_commit(&self, new_commit_sha: &str) -> anyhow::Result<()> {
        let json = json!({ "sha": new_commit_sha });
        let endpoint = format!("git/refs/heads/{}", self.config.branch);
        self.handle_request(&endpoint, Method::PATCH, Some(json)).await?;
        Ok(())
    }

    /// Helper to send a request to the GitHub API.
    async fn handle_request(
        &self,
        endpoint: &str,
        method: Method,
        body: Option<Value>,
    ) -> anyhow::Result<Value> {
        let url = format!(
            "{}/{}/{}/{}",
            self.config.api_url, self.config.owner_repo, self.config.owner_repo, endpoint
        );
        let mut req = self
            .client
            .request(method, &url)
            .bearer_auth(&self.config.token)
            .header("User-Agent", "rust-readme-chess");

        if let Some(json_body) = body {
            req = req.json(&json_body);
        }

        let resp = req.send().await?;
        let text = resp.text().await?;
        let json: Value = serde_json::from_str(&text)?;
        Ok(json)
    }

    /// Fetch the current README content from GitHub
    pub async fn fetch_readme(&self) -> anyhow::Result<String> {
        let url = format!(
            "https://api.github.com/repos/{}/contents/{}?ref={}",
            self.config.owner_repo, self.config.readme_path, self.config.branch
        );
        let resp = self
            .client
            .get(&url)
            .bearer_auth(&self.config.token)
            .header("User-Agent", "rust-readme-chess")
            .send()
            .await?
            .json::<serde_json::Value>()
            .await?;
        let content_b64 = resp["content"].as_str().unwrap_or("");
        let content = BASE64_STANDARD
            .decode(content_b64.replace('\n', ""))
            .map(|bytes| String::from_utf8_lossy(&bytes).to_string())
            .unwrap_or_default();
        Ok(content)
    }

    /// Poll until the README matches the expected content or timeout
    pub async fn poll_readme_until_updated(&self, expected: &str, max_attempts: usize) -> bool {
        for _ in 0..max_attempts {
            if let Ok(current) = self.fetch_readme().await {
                if current.trim() == expected.trim() {
                    return true;
                }
            }
        }
        false
    }
}
