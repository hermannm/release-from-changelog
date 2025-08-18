use anyhow::Result;
use release_from_changelog::{create_github_release_for_changelog_entry, ActionInput};

#[test]
fn creates_release_from_changelog() -> Result<()> {
    let mut server = mockito::Server::new();
    let server_url = server.url();

    let github_server_mock = server.mock("POST", "/repos/hermannm/gruvbox-plain/releases")
        .match_body(r#"{"tag_name":"v0.4.0","name":"v0.4.0","body":"- Overhaul UI colors in IntelliJ to improve consistency\n- Improve IntelliJ syntax highlighting for:\n    - TypeScript/JavaScript\n    - C/C++\n    - Markdown\n    - XML\n- Improve VSCode syntax highlighting for Rust"}"#)
        .match_header("Accept", "application/vnd.github+json")
        .match_header("Authorization", format!("Bearer: {TEST_TOKEN}").as_str())
        .match_header("X-GitHub-Api-Version", "2022-11-28")
        .match_header("User-Agent", "hermannm")
        .with_status(201)
        .with_body(GITHUB_CREATE_RELEASE_RESPONSE)
        .create();

    let release = create_github_release_for_changelog_entry(&ActionInput {
        tag_name: "v0.4.0".to_string(),
        release_title: None,
        changelog_file_path: Some("tests/testdata/CHANGELOG_1.md".to_string()),
        repo_name: "gruvbox-plain".to_string(),
        repo_owner: "hermannm".to_string(),
        auth_token: TEST_TOKEN.to_string(),
        api_url: server_url,
    })?;

    assert_eq!("v0.4.0", &release.name);
    assert_eq!(
        "https://github.com/hermannm/gruvbox-plain/releases/v0.4.0",
        &release.url
    );

    github_server_mock.assert(); // Assert that mock server was hit

    Ok(())
}

static TEST_TOKEN: &str = "test-token";

/// Based on example response from:
/// https://docs.github.com/en/rest/releases/releases?apiVersion=2022-11-28#create-a-release
static GITHUB_CREATE_RELEASE_RESPONSE: &str = r#"{
  "url": "https://api.github.com/repos/hermannm/gruvbox-plain/releases/4",
  "html_url": "https://github.com/hermannm/gruvbox-plain/releases/v0.4.0",
  "assets_url": "https://api.github.com/repos/hermannm/gruvbox-plain/releases/4/assets",
  "upload_url": "https://uploads.github.com/repos/hermannm/gruvbox-plain/releases/4/assets{?name,label}",
  "tarball_url": "https://api.github.com/repos/hermannm/gruvbox-plain/tarball/v0.4.0",
  "zipball_url": "https://api.github.com/repos/hermannm/gruvbox-plain/zipball/v0.4.0",
  "discussion_url": "https://github.com/hermannm/gruvbox-plain/discussions/90",
  "id": 1,
  "node_id": "MDc6UmVsZWFzZTE=",
  "tag_name": "v0.4.0",
  "target_commitish": "master",
  "name": "v0.4.0",
  "body": "Description of the release",
  "draft": false,
  "prerelease": false,
  "immutable": false,
  "created_at": "2013-02-27T19:35:32Z",
  "published_at": "2013-02-27T19:35:32Z",
  "author": {
    "login": "octocat",
    "id": 1,
    "node_id": "MDQ6VXNlcjE=",
    "avatar_url": "https://github.com/images/error/octocat_happy.gif",
    "gravatar_id": "",
    "url": "https://api.github.com/users/octocat",
    "html_url": "https://github.com/octocat",
    "followers_url": "https://api.github.com/users/octocat/followers",
    "following_url": "https://api.github.com/users/octocat/following{/other_user}",
    "gists_url": "https://api.github.com/users/octocat/gists{/gist_id}",
    "starred_url": "https://api.github.com/users/octocat/starred{/owner}{/repo}",
    "subscriptions_url": "https://api.github.com/users/octocat/subscriptions",
    "organizations_url": "https://api.github.com/users/octocat/orgs",
    "repos_url": "https://api.github.com/users/octocat/repos",
    "events_url": "https://api.github.com/users/octocat/events{/privacy}",
    "received_events_url": "https://api.github.com/users/octocat/received_events",
    "type": "User",
    "site_admin": false
  },
  "assets": [
    {
      "url": "https://api.github.com/repos/hermannm/gruvbox-plain/releases/assets/1",
      "browser_download_url": "https://github.com/hermannm/gruvbox-plain/releases/download/v0.4.0/example.zip",
      "id": 1,
      "node_id": "MDEyOlJlbGVhc2VBc3NldDE=",
      "name": "example.zip",
      "label": "short description",
      "state": "uploaded",
      "content_type": "application/zip",
      "size": 1024,
      "digest": "sha256:2151b604e3429bff440b9fbc03eb3617bc2603cda96c95b9bb05277f9ddba255",
      "download_count": 42,
      "created_at": "2013-02-27T19:35:32Z",
      "updated_at": "2013-02-27T19:35:32Z",
      "uploader": {
        "login": "octocat",
        "id": 1,
        "node_id": "MDQ6VXNlcjE=",
        "avatar_url": "https://github.com/images/error/octocat_happy.gif",
        "gravatar_id": "",
        "url": "https://api.github.com/users/octocat",
        "html_url": "https://github.com/octocat",
        "followers_url": "https://api.github.com/users/octocat/followers",
        "following_url": "https://api.github.com/users/octocat/following{/other_user}",
        "gists_url": "https://api.github.com/users/octocat/gists{/gist_id}",
        "starred_url": "https://api.github.com/users/octocat/starred{/owner}{/repo}",
        "subscriptions_url": "https://api.github.com/users/octocat/subscriptions",
        "organizations_url": "https://api.github.com/users/octocat/orgs",
        "repos_url": "https://api.github.com/users/octocat/repos",
        "events_url": "https://api.github.com/users/octocat/events{/privacy}",
        "received_events_url": "https://api.github.com/users/octocat/received_events",
        "type": "User",
        "site_admin": false
      }
    }
  ]
}"#;
