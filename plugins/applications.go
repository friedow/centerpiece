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

func newApplicationsPluginOptions() []PluginOption {
	applications := []*application{}

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
			application := newApplication(desktopEntry.Name, path)
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

func filterDuplicateApplications(applications []*application) []*application {
	applicationsMap := map[string]*application{}
	for _, application := range applications {
		applicationsMap[application.title] = application
	}

	uniqueApplications := []*application{}
	for _, application := range applicationsMap {
		uniqueApplications = append(uniqueApplications, application)
	}

	return uniqueApplications
}

type application struct {
	*options.TextOption

	title string
	path  string
}

var _ PluginOption = application{}

func newApplication(title string, path string) *application {
	this := application{}

	this.TextOption = options.NewTextOption(title, "Enter to launch")

	this.title = title
	this.path = path

	return &this
}

func (this application) PluginName() string {
	return "Applications"
}

func (this application) OnActivate() {
	exec.Command("xdg-open", this.path).Run()
}

func (this application) IsVisible(queryPart string) bool {
	return strings.Contains(strings.ToLower(this.title), queryPart)
}
