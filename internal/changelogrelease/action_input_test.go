package changelogrelease

import (
	"os"
	"sync"
	"testing"

	"hermannm.dev/opt"
)

func TestActionInputFromEnv(t *testing.T) {
	setTestEnv(
		t,
		map[string]string{
			"INPUT_TAG_NAME":       "v0.4.0",
			"GITHUB_REF":           "refs/tags/v0.3.0", // Should be ignored when INPUT_TAG is set
			"INPUT_RELEASE_TITLE":  "Release 0.4.0",
			"INPUT_CHANGELOG_PATH": "dir/CHANGELOG.md",
			"GITHUB_REPOSITORY":    "hermannm/release-from-changelog",
			"INPUT_TOKEN":          "test-token",
			"GITHUB_API_URL":       "https://api.github.com",
		},
		func() {
			input, err := ActionInputFromEnv()
			assertNilError(t, err)

			expected := ActionInput{
				TagName:           "v0.4.0",
				ReleaseTitle:      opt.Value("Release 0.4.0"),
				ChangelogFilePath: opt.Value("dir/CHANGELOG.md"),
				RepoName:          "release-from-changelog",
				RepoOwner:         "hermannm",
				AuthToken:         "test-token",
				ApiURL:            "https://api.github.com",
			}
			assertDeepEqual(t, input, expected, "action input from env")
		},
	)
}

func TestOptionalInputsAndFallback(t *testing.T) {
	setTestEnv(
		t,
		map[string]string{
			// When INPUT_TAG_NAME is not set, the tag name should be parsed from this env var
			"GITHUB_REF":        "refs/tags/v0.3.0",
			"GITHUB_REPOSITORY": "hermannm/release-from-changelog",
			"INPUT_TOKEN":       "test-token",
			"GITHUB_API_URL":    "https://api.github.com",
		},
		func() {
			input, err := ActionInputFromEnv()
			assertNilError(t, err)

			expected := ActionInput{
				TagName:           "v0.3.0",
				ReleaseTitle:      opt.Empty[string](),
				ChangelogFilePath: opt.Empty[string](),
				RepoName:          "release-from-changelog",
				RepoOwner:         "hermannm",
				AuthToken:         "test-token",
				ApiURL:            "https://api.github.com",
			}
			assertDeepEqual(t, input, expected, "action input from env")
		},
	)
}

func setTestEnv(
	t *testing.T,
	envVars map[string]string,
	testFunc func(),
) {
	testEnvLock.Lock()
	defer testEnvLock.Unlock()

	previousValues := make(map[string]opt.Option[string], len(envVars))

	for key, value := range envVars {
		previousValue := os.Getenv(key)

		err := os.Setenv(key, value)
		assertNilError(t, err)

		if previousValue == "" {
			previousValues[key] = opt.Empty[string]()
		} else {
			previousValues[key] = opt.Value(previousValue)
		}
	}

	testFunc()

	for key, previousValue := range previousValues {
		if previousValue, ok := previousValue.Get(); ok {
			err := os.Setenv(key, previousValue)
			assertNilError(t, err)
		} else {
			err := os.Unsetenv(key)
			assertNilError(t, err)
		}
	}
}

var testEnvLock = new(sync.Mutex)
