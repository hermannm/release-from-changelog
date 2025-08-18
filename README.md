# release-from-changelog

GitHub Action for creating a GitHub release from a `CHANGELOG.md` entry on the
["Keep a Changelog" format](https://keepachangelog.com/).

## Developer's guide

When publishing a new release:

- Bump the `runs.image` version in `action.yml`
- Bump `hermannm/release-from-changelog` under `create-release` in `.github/workflows/release.yml`
  to the current latest version of the action
    - We use our own action to create releases, but obviously we can't use the version that we're
      currently publishing
- Add an entry to `CHANGELOG.md`
- Create a new tag on the format `vX.Y.Z`, and push it
    - Our release workflow will then run the tests, build the Docker image for the action, and
      publish a release with the changelog 
