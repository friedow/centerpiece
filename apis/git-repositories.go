package apis

import (
	"friedow/tucan-search/models"
	"io/fs"
	"os"
	"path/filepath"
	"strings"
)

var GitRepositoriesApi *gitRepositoriesApi = newGitRepositoriesApi()

type gitRepositoriesApi struct {
	cache
}

func newGitRepositoriesApi() *gitRepositoriesApi {
	this := gitRepositoriesApi{
		cache: newCache(),
	}

	return &this
}

func (this *gitRepositoriesApi) GetGitRepositories() []*models.GitRepository {
	home := os.Getenv("HOME")
	gitRepositories := []*models.GitRepository{}

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
				gitRepository := models.NewGitRepository(gitRepositoryTitle, gitRepositoryPath)
				gitRepositories = append(gitRepositories, gitRepository)
				return nil
			}

			return nil
		},
	)

	return gitRepositories
}
