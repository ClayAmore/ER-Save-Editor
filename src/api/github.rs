use reqwest::blocking::Client;
use super::models::releases::Release;


pub struct GithubApi;

impl GithubApi {
    /// Fetch latest release data using github api
    pub fn fetch_latest_release(user: &str, repo: &str) -> Result<Release, reqwest::Error> {
        let url = &format!("https://api.github.com/repos/{user}/{repo}/releases/latest");
        let client = Client::new();
        let release = client
            .get(url)
            .header("User-Agent", "request")
            .send()?
            .json::<Release>()?;

        Ok(release)
    }
}