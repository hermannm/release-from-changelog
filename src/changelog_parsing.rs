use crate::utils::IsBlank;
use anyhow::{bail, Context, Result};
use regex::Regex;
use std::borrow::Cow;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::sync::LazyLock;

pub fn get_changelog_entry(changelog_file_path: &Path, version_to_find: &str) -> Result<String> {
    let file = File::open(changelog_file_path).with_context(|| {
        format!("Failed to open changelog file at path '{changelog_file_path:?}'")
    })?;
    let reader = BufReader::new(file);

    let mut changelog_entry_lines = Vec::<String>::new();
    let mut found_changelog_entry = false;
    let target_titles = get_target_titles(version_to_find);

    let mut line_iterator = reader.lines();
    while let Some(line_result) = line_iterator.next() {
        let line = line_result.context("Failed to read line from changelog file")?;

        if !found_changelog_entry {
            if target_titles.iter().any(|title| line.starts_with(title)) {
                found_changelog_entry = true;

                match line_iterator.next() {
                    Some(Ok(next_line)) => {
                        // If line after title is blank, we don't want to include it in the
                        // changelog
                        if !next_line.is_blank() {
                            changelog_entry_lines.push(next_line)
                        }
                    }
                    Some(Err(err)) => {
                        return Err(err).context("Failed to read line from changelog file");
                    }
                    None => {
                        bail!("Unexpected end of changelog file after title line '{line}'");
                    }
                }
            }

            continue;
        }

        if changelog_entry_ended(&line) {
            break;
        }

        changelog_entry_lines.push(line);
    }

    if !found_changelog_entry {
        let [target_title_1, target_title_2] = target_titles;
        bail!("No changelog entry found for version '{version_to_find}' in changelog file '{changelog_file_path:?}' (looking for titles starting with '{target_title_1}' or '{target_title_2}')");
    }

    // Remove trailing blank lines from changelog
    while changelog_entry_lines
        .last()
        .is_some_and(|last_line| last_line.is_blank())
    {
        changelog_entry_lines.pop();
    }

    if changelog_entry_lines.is_empty() {
        bail!("Changelog entry for version '{version_to_find}' was empty")
    }

    Ok(changelog_entry_lines.join("\n"))
}

fn get_target_titles(version_to_find: &str) -> [String; 2] {
    let version_without_prefix: &str;
    let version_with_prefix: Cow<str>;

    match version_to_find.strip_prefix('v') {
        Some(version) => {
            version_without_prefix = version;
            version_with_prefix = Cow::Borrowed(version_to_find);
        }
        None => {
            version_without_prefix = version_to_find;
            version_with_prefix = Cow::Owned(format!("v{version_to_find}"));
        }
    }

    [
        format!("## [{version_without_prefix}]"),
        format!("## [{version_with_prefix}]"),
    ]
}

/// A changelog entry has ended if we find:
/// - A higher-level title (#)
/// - A new changelog entry at the same title level (##)
/// - The start of the link section at the end of the changelog
///   - Example: [v0.1.0]: <link>
fn changelog_entry_ended(line: &str) -> bool {
    static TAG_LINK_REGEX: LazyLock<Regex> = LazyLock::new(||
            // Regex:
            // - Leading ^ to match beginning of line
            // - \[ and \] to match square brackets around link text
            // - [^\[\]]+ to match link text: all characters _except_ [ or ]
            // - : to match trailing colon
            Regex::new(r#"^\[[^\[\]]+\]:"#).expect("Should be valid regex"));

    line.starts_with("# ") || line.starts_with("## ") || TAG_LINK_REGEX.is_match(&line)
}
