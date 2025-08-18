use crate::{
    changelog_parsing::get_changelog_entry, github_release::GitHubApiClient,
    utils::get_optional_env_var, utils::get_required_env_var,
};
use anyhow::bail;
use anyhow::Context;
use regex::Regex;
use std::path::Path;
use std::sync::LazyLock;

pub mod changelog_parsing;
pub mod github_release;
mod utils;

pub fn create_github_release_for_changelog_entry(
    input: &ActionInput,
) -> anyhow::Result<CreatedRelease> {
    // Validate that tag name follows semantic versioning
    validate_tag_name(&input.tag_name)?;

    // If release name is not set, use tag name
    let release_name = input.release_name.as_deref().unwrap_or(&input.tag_name);
    // If changelog file path is not set, default to 'CHANGELOG.md' at root
    let changelog_file_path = input
        .changelog_file_path
        .as_deref()
        .unwrap_or("CHANGELOG.md");

    let changelog = get_changelog_entry(Path::new(changelog_file_path), &input.tag_name)
        .context("Failed to get changelog entry")?;

    let github_client = GitHubApiClient::new(&input.api_url);
    let release = github_client
        .create_github_release(
            &input.tag_name,
            &release_name,
            &changelog,
            &input.repo_name,
            &input.repo_owner,
            &input.auth_token,
        )
        .context("Failed to create GitHub release")?;

    Ok(release)
}

#[derive(PartialEq, Eq, Debug)]
pub struct ActionInput {
    pub tag_name: String,
    pub release_name: Option<String>,
    pub changelog_file_path: Option<String>,
    pub repo_name: String,
    pub repo_owner: String,
    pub auth_token: String,
    pub api_url: String,
}

impl ActionInput {
    pub fn from_env() -> anyhow::Result<ActionInput> {
        // We allow tag refs to be passed explicitly as 'tag_name', but fall back to the
        // 'GITHUB_REF' environment variable set by GitHub.
        let tag_name = match get_optional_env_var("INPUT_TAG_NAME") {
            Some(tag_name) => tag_name,
            None => {
                let tag_ref = get_required_env_var("GITHUB_REF")?;
                // We expect this prefix for tag refs. See GitHub docs for GITHUB_REF:
                // https://docs.github.com/en/actions/reference/workflows-and-actions/variables
                tag_ref.strip_prefix("refs/tags/")
                    .with_context(||
                        format!("Expected 'GITHUB_REF' environment variable to be on the format 'refs/tags/<tag_name>', but got '{tag_ref}'")
                    )?
                    .to_owned()
            }
        };

        let repo = get_required_env_var("GITHUB_REPOSITORY")?;
        let (repo_owner, repo_name) = repo.split_once('/')
            .with_context(||
                format!("Expected 'GITHUB_REPOSITORY' environment variable to be on the format 'repo_owner/repo_name', but got '{repo}'")
            )?;

        // GitHub token may be set explicitly through INPUT_TOKEN, but if not we fall back to
        // GITHUB_TOKEN
        let auth_token = match get_optional_env_var("INPUT_TOKEN") {
            Some(token) => token,
            None => get_required_env_var("GITHUB_TOKEN")?,
        };

        Ok(ActionInput {
            tag_name,
            release_name: get_optional_env_var("INPUT_RELEASE_NAME"),
            changelog_file_path: get_optional_env_var("INPUT_CHANGELOG_PATH"),
            repo_name: repo_name.to_owned(),
            repo_owner: repo_owner.to_owned(),
            auth_token,
            api_url: get_required_env_var("GITHUB_API_URL")?,
        })
    }
}

pub struct CreatedRelease {
    pub name: String,
    pub url: String,
}

fn validate_tag_name(tag_name: &str) -> anyhow::Result<()> {
    let version = tag_name.strip_prefix('v').unwrap_or(tag_name);

    static SEMANTIC_VERSIONING_REGEX: LazyLock<Regex> = LazyLock::new(|| {
        // Regex:
        // - Leading ^ and trailing $, so we always match the full string
        // - \d+ to match at least 1 digit
        // - \. to match dots between digits
        Regex::new(r#"^\d+\.\d+\.\d+$"#).expect("Should be valid regex")
    });

    if !SEMANTIC_VERSIONING_REGEX.is_match(version) {
        bail!("Invalid tag '{tag_name}': Expected semantic version format X.Y.Z (with optional leading 'v')")
    }

    Ok(())
}
