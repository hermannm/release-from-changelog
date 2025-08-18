use anyhow::Result;
use release_from_changelog::ActionInput;
use std::env;
use std::sync::{LazyLock, Mutex};

#[test]
fn gets_action_input_from_env() -> Result<()> {
    set_test_env(
        &[
            ("INPUT_TAG_NAME", "v0.4.0"),
            ("GITHUB_REF", "refs/tags/v0.3.0"), // Should be ignored when INPUT_TAG is set
            ("INPUT_RELEASE_NAME", "Release 0.4.0"),
            ("INPUT_CHANGELOG_PATH", "dir/CHANGELOG.md"),
            ("GITHUB_REPOSITORY", "hermannm/release-from-changelog"),
            ("INPUT_TOKEN", "test-token"),
            ("GITHUB_API_URL", "https://api.github.com"),
        ],
        || {
            let input = ActionInput::from_env()?;

            assert_eq!(input, ActionInput {
                tag_name: "v0.4.0".to_string(),
                release_name: Some("Release 0.4.0".to_string()),
                changelog_file_path: Some("dir/CHANGELOG.md".to_string()),
                repo_name: "release-from-changelog".to_string(),
                repo_owner: "hermannm".to_string(),
                auth_token: "test-token".to_string(),
                api_url: "https://api.github.com".to_string(),
            });

            Ok(())
        },
    )
}

#[test]
fn optional_inputs_and_fallback() -> Result<()> {
    set_test_env(
        &[
            // When INPUT_TAG_NAME is not set, the tag name should be parsed from this env var
            ("GITHUB_REF", "refs/tags/v0.3.0"),
            ("GITHUB_REPOSITORY", "hermannm/release-from-changelog"),
            ("INPUT_TOKEN", "test-token"),
            ("GITHUB_API_URL", "https://api.github.com"),
        ],
        || {
            let input = ActionInput::from_env()?;

            assert_eq!(input, ActionInput {
                tag_name: "v0.3.0".to_string(),
                release_name: None,
                changelog_file_path: None,
                repo_name: "release-from-changelog".to_string(),
                repo_owner: "hermannm".to_string(),
                auth_token: "test-token".to_string(),
                api_url: "https://api.github.com".to_string(),
            });

            Ok(())
        },
    )
}

fn set_test_env<ReturnT>(env_vars: &[(&str, &str)], test_block: impl FnOnce() -> ReturnT) -> ReturnT {
    // We put a lock around the test block, since environment variables are global state, so we
    // cannot run these tests in parallel
    static LOCK: LazyLock<Mutex<()>> = LazyLock::new(|| Mutex::new(()));
    let lock_guard = LOCK.lock().unwrap();

    let mut previous_values = Vec::<(&str, Option<String>)>::with_capacity(env_vars.len());

    for (key, value) in env_vars {
        let previous_value = env::var(key).ok();
        env::set_var(key, value);
        previous_values.push((key, previous_value));
    }

    let result = test_block();

    for (key, previous_value) in previous_values {
        if let Some(previous_value) = previous_value {
            env::set_var(key, previous_value);
        } else {
            env::remove_var(key);
        }
    }

    drop(lock_guard);

    return result;
}