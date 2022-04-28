package plugins

import (
	"bufio"
	"friedow/tucan-search/components/options"
	"io/fs"
	"os"
	"os/exec"
	"path/filepath"
	"strings"

	"github.com/rkoesters/xdg/basedir"
	"github.com/rkoesters/xdg/desktop"
)

func NewApplicationsPluginOptions() []PluginOption {
	applications := []*Application{}

	for _, dataDir := range basedir.DataDirs {
		err := filepath.WalkDir(dataDir+"/applications", func(path string, info fs.DirEntry, _ error) error {
			if !strings.HasSuffix(path, ".desktop") {
				return nil
			}

			file, err := os.Open(path)
			if err != nil {
				panic(err)
			}

			reader := bufio.NewReader(file)
			desktopEntry, _ := desktop.New(reader)
			application := NewApplication(desktopEntry.Name, path)
			applications = append(applications, application)

			return nil
		})

		if err != nil {
			panic(err)
		}
	}

	uniqueApplications := filterDuplicateApplications(applications)

	pluginOptions := []PluginOption{}
	for _, uniqueApplication := range uniqueApplications {
		pluginOptions = append(pluginOptions, uniqueApplication)
	}
	return pluginOptions
}

func filterDuplicateApplications(applications []*Application) []*Application {
	applicationsMap := map[string]*Application{}
	for _, application := range applications {
		applicationsMap[application.title] = application
	}

	uniqueApplications := []*Application{}
	for _, application := range applicationsMap {
		uniqueApplications = append(uniqueApplications, application)
	}

	return uniqueApplications
}

type Application struct {
	*options.TextOption

	title string
	path  string
}

var _ PluginOption = Application{}

func NewApplication(title string, path string) *Application {
	this := Application{}

	this.TextOption = options.NewTextOption(title, "Enter to launch")

	this.title = title
	this.path = path

	return &this
}

func (this Application) PluginName() string {
	return "Applications"
}

func (this Application) OnActivate() {
	exec.Command("xdg-open", this.path).Run()
}

func (this Application) IsVisible(queryPart string) bool {
	return strings.Contains(strings.ToLower(this.title), queryPart)
}
