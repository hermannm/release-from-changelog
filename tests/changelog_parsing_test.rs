use anyhow::Result;
use release_from_changelog::changelog_parsing::get_changelog_entry;
use std::path::Path;

#[test]
fn version_with_leading_v_matches_changelog_without_v() -> Result<()> {
    let changelog_entry =
        get_changelog_entry(&Path::new("tests/testdata/CHANGELOG_2.md"), "v0.2.0")?;
    assert_eq!(changelog_entry, "- Version without leading 'v'");

    Ok(())
}

#[test]
fn version_without_leading_v_matches_changelog_with_v() -> Result<()> {
    let changelog_entry =
        get_changelog_entry(&Path::new("tests/testdata/CHANGELOG_2.md"), "0.3.0")?;
    assert_eq!(changelog_entry, "- Test");

    Ok(())
}

#[test]
fn parses_changelog_at_end_of_file() -> Result<()> {
    let changelog_entry =
        get_changelog_entry(&Path::new("tests/testdata/CHANGELOG_2.md"), "v0.1.0")?;
    assert_eq!(changelog_entry, "- Changelog entry at end of file");

    Ok(())
}