package models

import (
	"os"
	"os/exec"
	"strings"
)

type GitRepository struct {
	title string
	path  string
}

func NewGitRepository(title string, path string) *GitRepository {
	this := GitRepository{
		title: title,
		path:  path,
	}

	return &this
}

func (this *GitRepository) OnActivate() {
	exec.Command("code", this.path).Output()
	os.Exit(0)
}

func (this *GitRepository) IsVisible(queryPart string) bool {
	return strings.Contains(strings.ToLower(this.title), queryPart)
}
