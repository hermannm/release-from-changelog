package changelogrelease

import (
	"context"
	"fmt"
	"hermannm.dev/wrap"
	"log/slog"
	"net/http"
	"regexp"
	"strings"
)

type CreatedRelease struct {
	Name string
	URL  string
}

func CreateGitHubReleaseForChangelogEntry(
	ctx context.Context,
	input ActionInput,
	httpClient *http.Client,
) (CreatedRelease, error) {
	if err := validateTagName(input.TagName); err != nil {
		return CreatedRelease{}, err
	}

	releaseTitle := input.ReleaseTitle.GetOrDefault(input.TagName)
	changelogPath := input.ChangelogFilePath.GetOrDefault("CHANGELOG.md")

	changelog, err := getChangelogEntry(changelogPath, input.TagName)
	if err != nil {
		return CreatedRelease{}, wrap.Error(err, "Failed to get changelog entry")
	}

	githubClient := GitHubApiClient{httpClient: httpClient, apiURL: input.ApiURL}
	release, err := githubClient.createRelease(
		ctx,
		input.TagName,
		releaseTitle,
		changelog,
		input.RepoName,
		input.RepoOwner,
		input.AuthToken,
	)
	if err != nil {
		return CreatedRelease{}, wrap.Error(err, "Failed to create GitHub release")
	}

	return release, nil
}

func validateTagName(tagName string) error {
	version := strings.TrimPrefix(tagName, "v")

	if !semanticVersioningRegex.MatchString(version) {
		return fmt.Errorf(
			"Invalid tag '%s': Expected semantic version format 'vX.Y.Z' (leading 'v' is optional)",
			tagName,
		)
	}

	return nil
}

// Regex:
// - Leading ^ and trailing $, so we always match the full string
// - \d+ to match at least 1 digit
// - \. to match dots between digits
var semanticVersioningRegex = regexp.MustCompile(`^[0-9]+\.[0-9]+\.[0-9]+$`)

// Implements [slog.LogValuer].
func (release CreatedRelease) LogValue() slog.Value {
	return slog.GroupValue(
		slog.String("name", release.Name),
		slog.String("url", release.URL),
	)
}
