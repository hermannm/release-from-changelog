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
        auth_token: &str,
    ) -> Result<CreatedRelease> {
        let url = format!("{}/repos/{repo_owner}/{repo_name}/releases", self.api_url);

        let request_body = CreateReleaseRequest {
            tag_name,
            name: release_title,
            body: changelog,
        };

        let response = self
            .http_client
            .post(url)
            .json(&request_body)
            .header("Accept", "application/vnd.github+json")
            .header("Authorization", format!("Bearer {auth_token}"))
            .header("X-GitHub-Api-Version", "2022-11-28")
            // GitHub API requires sending a User-Agent header, and they recommend setting it to
            // your GitHub username:
            // https://docs.github.com/en/rest/using-the-rest-api/getting-started-with-the-rest-api#user-agent
            // In this case, that will be the `repo_owner`.
            .header("User-Agent", repo_owner)
            .send()
            .context("Failed to send create release request to GitHub")?;

        if !response.status().is_success() {
            let response_status = response.status();

            let (body_prefix, response_body): (&str, String) = match response.text() {
                Err(_) => ("and failed to read response body", "".to_owned()),
                Ok(body) if body.is_blank() => ("with blank response body", "".to_owned()),
                Ok(body) => ("response_body:\n", body),
            };
            bail!(
                "Got unsuccessful response ({response_status}) from GitHub when trying to create release, {body_prefix}{response_body}",
            )
        }

        let response_body = response.json::<CreateReleaseResponse>()
            .context("GitHub create release request succeeded, but failed to get release URL from response body")?;

        Ok(CreatedRelease {
            name: release_title.to_owned(),
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
