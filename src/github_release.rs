use crate::utils::IsBlank;
use crate::CreatedRelease;
use anyhow::{bail, Context, Result};
use serde::{Deserialize, Serialize};

pub(crate) struct GitHubApiClient<'a> {
    api_url: &'a str,
    http_client: reqwest::blocking::Client,
}

impl GitHubApiClient<'_> {
    pub(crate) fn new<'a>(api_url: &'a str) -> GitHubApiClient<'a> {
        GitHubApiClient {
            api_url,
            http_client: reqwest::blocking::Client::new(),
        }
    }

    pub(crate) fn create_github_release(
        &self,
        tag_name: &str,
        release_title: &str,
        changelog: &str,
        repo_name: &str,
        repo_owner: &str,
        github_token: &str,
    ) -> Result<CreatedRelease> {
        let url = format!("{}/repos/{repo_owner}/{repo_name}/releases", self.api_url);

        let body = CreateReleaseRequest {
            tag_name,
            name: release_title,
            body: changelog,
        };

        let response = self
            .http_client
            .post(url)
            .json(&body)
            .header("Accept", "application/vnd.github+json")
            .header("Authorization", format!("Bearer: {github_token}"))
            .header("X-GitHub-Api-Version", "2022-11-28")
            .send()
            .context("Failed to send create release request to GitHub")?;

        if !response.status().is_success() {
            let response_status = response.status();
            let response_body = match response.text() {
                Err(_) => "<failed to read>".to_string(),
                Ok(text) if text.is_blank() => "<blank>".to_string(),
                Ok(text) => text,
            };
            bail!(
                "Got unsuccessful response ({response_status}) from GitHub when trying to create release, response body: {response_body}",
            )
        }

        let response_body = response.json::<CreateReleaseResponse>()
            .context("GitHub create release request succeeded, but failed to get release URL from response body")?;

        Ok(CreatedRelease {
            name: release_title.to_string(),
            url: response_body.html_url,
        })
    }
}

#[derive(Serialize)]
struct CreateReleaseRequest<'a> {
    tag_name: &'a str,
    name: &'a str,
    body: &'a str,
}

#[derive(Deserialize)]
struct CreateReleaseResponse {
    html_url: String,
}
