package changelogrelease

import (
	"fmt"
	"hermannm.dev/opt"
	"os"
	"strings"
)

type ActionInput struct {
	TagName           string
	ReleaseTitle      opt.Option[string]
	ChangelogFilePath opt.Option[string]
	RepoName          string
	RepoOwner         string
	AuthToken         string
	ApiURL            string
}

func ActionInputFromEnv() (ActionInput, error) {
	tagName, err := getTagNameFromEnv()
	if err != nil {
		return ActionInput{}, err
	}

	repoOwner, repoName, err := getRepoOwnerAndNameFromEnv()
	if err != nil {
		return ActionInput{}, err
	}

	authToken, err := getRequiredEnvVar("INPUT_TOKEN")
	if err != nil {
		return ActionInput{}, err
	}

	apiUrl, err := getRequiredEnvVar("GITHUB_API_URL")
	if err != nil {
		return ActionInput{}, err
	}

	return ActionInput{
		TagName:           tagName,
		ReleaseTitle:      getOptionalEnvVar("INPUT_RELEASE_TITLE"),
		ChangelogFilePath: getOptionalEnvVar("INPUT_CHANGELOG_PATH"),
		RepoName:          repoName,
		RepoOwner:         repoOwner,
		AuthToken:         authToken,
		ApiURL:            apiUrl,
	}, nil
}

func getTagNameFromEnv() (string, error) {
	tagName, ok := getOptionalEnvVar("INPUT_TAG_NAME").Get()
	if !ok {
		tagRef, err := getRequiredEnvVar("GITHUB_REF")
		if err != nil {
			return "", err
		}

		if !strings.HasPrefix(tagRef, "refs/tags/") {
			return "", fmt.Errorf(
				"Expected 'GITHUB_REF' environment variable to be on the format 'refs/tags/<tag_name>', but got '%s'",
				tagRef,
			)
		}

		tagName = strings.TrimPrefix(tagRef, "refs/tags/")
	}
	return tagName, nil
}

func getRepoOwnerAndNameFromEnv() (repoOwner string, repoName string, err error) {
	repo, err := getRequiredEnvVar("GITHUB_REPOSITORY")
	if err != nil {
		return "", "", err
	}

	repoSplit := strings.SplitN(repo, "/", 2)
	if len(repoSplit) != 2 {
		return "", "", fmt.Errorf(
			"Expected 'GITHUB_REPOSITORY' environment variable to be on the format 'repo_owner/repo_name', but got '%s'",
			repo,
		)
	}

	return repoSplit[0], repoSplit[1], nil
}

func getRequiredEnvVar(name string) (value string, err error) {
	value = os.Getenv(name)
	if value == "" {
		return "", fmt.Errorf("Expected '%s' environment variable to be set", name)
	}
	return value, nil
}

func getOptionalEnvVar(name string) opt.Option[string] {
	value := os.Getenv(name)
	if value == "" {
		return opt.Empty[string]()
	}
	return opt.Value(value)
}
