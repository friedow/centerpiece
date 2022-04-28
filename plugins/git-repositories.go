package plugins

import (
	"friedow/tucan-search/components/options"
	"io/fs"
	"os"
	"os/exec"
	"path/filepath"
	"strings"
)

func NewGitRepositoriesPluginOptions() []PluginOption {
	home := os.Getenv("HOME")
	gitRepositories := []*GitRepository{}

	filepath.WalkDir(home,
		func(path string, info fs.DirEntry, err error) error {
			// bubble errors
			if err != nil {
				return err
			}

			// Skip hidden directories
			pathParts := strings.Split(path, "/")
			if len(pathParts) >= 2 {
				parentDirIndex := len(pathParts) - 2
				parentDir := pathParts[parentDirIndex]

				if strings.HasPrefix(parentDir, ".") {
					return fs.SkipDir
				}
			}

			// Add git directories to list
			if strings.HasSuffix(path, ".git") {
				gitRepositoryPath := strings.TrimSuffix(path, "/.git")
				gitRepositoryTitle := strings.Replace(gitRepositoryPath, home, "~", 1)
				gitRepository := NewGitRepository(gitRepositoryTitle, gitRepositoryPath)
				gitRepositories = append(gitRepositories, &gitRepository)
				return nil
			}

			return nil
		},
	)

	pluginOptions := []PluginOption{}
	for _, gitRepository := range gitRepositories {
		pluginOptions = append(pluginOptions, gitRepository)
	}
	return pluginOptions
}

type GitRepository struct {
	*options.TextOption

	title string
	path  string
}

var _ PluginOption = GitRepository{}

func NewGitRepository(title string, path string) GitRepository {
	this := GitRepository{}

	this.TextOption = options.NewTextOption(title, "Enter to open")

	this.title = title
	this.path = path

	return this
}

func (this GitRepository) PluginName() string {
	return "Git Repositories"
}

func (this GitRepository) OnActivate() {
	exec.Command("code", this.path).Output()
}

func (this GitRepository) IsVisible(queryPart string) bool {
	return strings.Contains(strings.ToLower(this.title), queryPart)
}
