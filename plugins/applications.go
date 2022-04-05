package plugins

import (
	"bufio"
	"friedow/tucan-search/models"
	"io/fs"
	"os"
	"path/filepath"
	"strings"

	"github.com/rkoesters/xdg/basedir"
	"github.com/rkoesters/xdg/desktop"
)

type ApplicationsPlugin struct{}

func (m ApplicationsPlugin) GetName() string {
	return "Applications"
}

func (m ApplicationsPlugin) GetOptionModels() []models.OptionModel {
	desktopEntries := getDesktopEntries()

	options := []models.OptionModel{}
	for _, desktopEntry := range desktopEntries {

		option := models.OptionModel{
			PluginName: m.GetName(),
			Title:      desktopEntry.Name,
			ActionText: "enter to launch",
			Data:       desktopEntry,
		}
		options = append(options, option)
	}

	return options
}

func (m ApplicationsPlugin) OnActivate(optionModel models.OptionModel) {
	// window := optionModel.Data.(I3MsgJsonPart)
	// focusWindowArgument := fmt.Sprintf("[con_id=%d] focus", window.Id)
	// exec.Command("i3-msg", focusWindowArgument).Run()
}

func getDesktopEntries() []*desktop.Entry {
	desktopEntries := []*desktop.Entry{}

	for _, dataDir := range basedir.DataDirs {
		err := filepath.WalkDir(dataDir+"/applications", func(path string, info fs.DirEntry, err error) error {
			// if err != nil {
			// 	panic(err)
			// }

			if !strings.HasSuffix(path, ".desktop") {
				return nil
			}

			file, err := os.Open(path)
			if err != nil {
				panic(err)
			}

			reader := bufio.NewReader(file)
			desktopEntry, _ := desktop.New(reader)
			desktopEntries = append(desktopEntries, desktopEntry)
			return nil
		})

		if err != nil {
			panic(err)
		}
	}

	return desktopEntries
}
