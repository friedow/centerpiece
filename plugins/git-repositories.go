package plugins

import (
	"io/fs"
	"log"
	"os"
	"path/filepath"
	"strings"

	"github.com/gotk3/gotk3/gdk"
	"github.com/gotk3/gotk3/gtk"
)

type GitRepositoriesPlugin struct{}

func (m GitRepositoriesPlugin) GetName() string {
	return "Git Repositories"
}

func (m GitRepositoriesPlugin) GetOptions() []*gtk.Box {

	gitRepositories := getGitRepositories()

	options := []*gtk.Box{}
	for _, gitRepository := range gitRepositories {
		option := OptionNew(gitRepository, "enter to open")
		options = append(options, option)
	}

	return options
}

func (m GitRepositoriesPlugin) OnKeyPressed(option *gtk.Box, key *gdk.Event) {
	log.Print("ehello")
}

func getGitRepositories() []string {
	home := os.Getenv("HOME")
	gitRepositories := []string{}

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
				gitRepository := strings.TrimSuffix(path, "/.git")
				gitRepository = strings.Replace(gitRepository, home, "~", 1)
				gitRepositories = append(gitRepositories, gitRepository)
				return nil
			}

			return nil
		})

	return gitRepositories
}
