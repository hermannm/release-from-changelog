# release-from-changelog

GitHub Action for creating a GitHub release from a `CHANGELOG.md` entry on the
["Keep a Changelog" format](https://keepachangelog.com/).

**Contents:**

- [Usage](#usage)
- [Developer's guide](#developers-guide)
- [Credits](#credits)

## Usage

- First, create a `CHANGELOG.md` file at the root of your repo, following the
  ["Keep a Changelog" format](https://keepachangelog.com/) (see the
  [changelog in this repo](https://github.com/hermannm/release-from-changelog/blob/main/CHANGELOG.md?plain=1)
  as an example)
    - This library expects changelog entry titles to start with `## [vX.Y.Z]` (the leading `v` is
      optional)
- Then, create a GitHub Actions release workflow (in `.github/workflows/release.yml`):

```yaml
name: Release
on:
  push:
    # Trigger release when a new semantic version tag (vX.Y.Z) is pushed
    tags:
      - v[0-9]+\.[0-9]+\.[0-9]+
jobs:
  create-release:
    name: Create release
    # `release-from-changelog` is a Docker action, which only works on Linux jobs
    runs-on: ubuntu-latest
    permissions:
      # Unfortunately, creating a release requires `contents: write` permission
      # https://github.com/orgs/community/discussions/68252
      contents: write
    steps:
      # https://github.com/actions/checkout
      - name: Checkout repository
        uses: actions/checkout@v5
      # Creates GitHub release from CHANGELOG.md entry for the pushed tag
      # https://github.com/hermannm/release-from-changelog
      - name: Create release from changelog
        uses: hermannm/release-from-changelog@v0.1.6
```

- When pushing a new tag on the format `vX.Y.Z`, this action will look for a corresponding entry in
  `CHANGELOG.md`, and automatically create a GitHub release for it!

You can customize the action with various inputs:

```yaml
- name: Create release from changelog
  uses: hermannm/release-from-changelog@v0.1.6
  with:
    # GitHub auth token to use for creating the release.
    # Uses the default `github.token` if not specified.
    # Note that the token needs `contents: write` permission in order to create the release:
    # https://github.com/orgs/community/discussions/68252
    token: <GitHub token>
    # Explicit name of the tag to create a release for.
    # If not specified, we get the tag name from the `GITHUB_REF` set by the workflow's trigger.
    # `GITHUB_REF` must be on the format `refs/tags/<tag_name>`, which it will be if `push: tags` is
    # used as the trigger for the workflow.
    tag_name: <name of existing tag on the format `vX.Y.Z` (leading `v` is optional)>
    # Title of the GitHub release to create.
    # Defaults to the tag name.
    release_title: <title>
    # Path to changelog file in this repo, which we parse to find an entry for the release tag.
    # Defaults to `CHANGELOG.md` at the root of the repo.
    changelog_path: <path/to/changelog.md>
```

## Developer's guide

When publishing a new release:

- Bump the `runs.image` version in `action.yml`
- Bump the version used in the example under [Usage](#usage) in the README
- Bump `hermannm/release-from-changelog` under `create-release` in `.github/workflows/release.yml`
  to the current latest version of the action
    - We use our own action to create releases, but obviously we can't use the version that we're
      currently publishing
- Add an entry to `CHANGELOG.md`
- Create a commit: `git commit -m "Release vX.Y.Z"`, and push it
- Create a tag: `git tag -a vX.Y.Z -m "Release vX.Y.Z"`, and push it with `git push --tags`
    - Our release workflow will then run the tests, build the Docker image for the action, and
      publish a release with the changelog

## Credits

Credits to [Dylan Anthony](https://github.com/dbanty) for their wonderful blog post
["How to Write a GitHub Action in Rust"](https://dylananthony.com/blog/how-to-write-a-github-action-in-rust/),
which was of great help when setting up the `Dockerfile`, `action.yml` and
`.github/workflows/release.yml` to deploy a Rust action with Docker.
