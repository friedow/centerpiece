package plugins

import (
	"fmt"
	"friedow/tucan-search/models"
	"io/fs"
	"log"
	"os"
	"os/exec"
	"path/filepath"
	"strings"
)

type GitRepositoriesPlugin struct{}

func (m GitRepositoriesPlugin) GetName() string {
	return "Git Repositories"
}

func (m GitRepositoriesPlugin) GetOptionModels() []models.OptionModel {
	gitRepositories := getGitRepositories()

	options := []models.OptionModel{}
	for _, gitRepository := range gitRepositories {

		option := models.OptionModel{
			PluginName: m.GetName(),
			Title:      gitRepository,
			ActionText: "enter to open",
		}
		options = append(options, option)
	}

	return options
}

func (m GitRepositoriesPlugin) OnActivate(optionModel models.OptionModel) {

	log.Print("activated", optionModel.Title)
	// let _result = Exec::cmd("code").arg(path).join();
	home := os.Getenv("HOME")
	repositoryPath := strings.Replace(optionModel.Title, "~", home, 1)
	command := fmt.Sprintf(`code "%s"`, repositoryPath)
	log.Print(command)
	exec.Command(command).Run()
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
