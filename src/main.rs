use anyhow::Context;
use release_from_changelog::{
    create_github_release_for_changelog_entry,
    ActionInput,
};
use std::ops::Deref;
use std::process::ExitCode;
use tracing::{error, info};

fn main() -> ExitCode {
    run_script(|| {
        let input = ActionInput::from_env()
            .context("Failed to get action input from environment variables")?;

        let release = create_github_release_for_changelog_entry(&input)?;

        info!(url = release.url, "Successfully created release '{}'", release.name);
        Ok(())
    })
}

#[inline]
#[must_use] // must_use, so ExitCode is not accidentally discarded
fn run_script(script: impl FnOnce() -> anyhow::Result<()>) -> ExitCode {
    devlog_tracing::subscriber().with_target(false).init();

    let result = script();
    match result {
        Ok(_) => ExitCode::SUCCESS,
        Err(error) => {
            error!(cause = error.deref());
            ExitCode::FAILURE
        }
    }
}
