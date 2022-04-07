package plugins

import (
	"bufio"
	"friedow/tucan-search/models"
	"io/fs"
	"log"
	"os"
	"os/exec"
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
			Title:      desktopEntry.entry.Name,
			ActionText: "enter to launch",
			Data:       desktopEntry,
		}
		options = append(options, option)
	}

	return options
}

func (m ApplicationsPlugin) OnActivate(optionModel models.OptionModel) {
	desktopEntry := optionModel.Data.(DesktopEntryWithPath)
	log.Print(desktopEntry)
	exec.Command("xdg-open", desktopEntry.path).Run()
}

type DesktopEntryWithPath struct {
	path  string
	entry *desktop.Entry
}

func getDesktopEntries() []DesktopEntryWithPath {
	desktopEntries := []DesktopEntryWithPath{}

	for _, dataDir := range basedir.DataDirs {
		err := filepath.WalkDir(dataDir+"/applications", func(path string, info fs.DirEntry, err error) error {
			if !strings.HasSuffix(path, ".desktop") {
				return nil
			}

			file, err := os.Open(path)
			if err != nil {
				panic(err)
			}

			reader := bufio.NewReader(file)
			desktopEntry, _ := desktop.New(reader)
			desktopEntryWithPath := DesktopEntryWithPath{
				path:  path,
				entry: desktopEntry,
			}
			desktopEntries = append(desktopEntries, desktopEntryWithPath)

			return nil
		})

		if err != nil {
			panic(err)
		}
	}

	return filterDuplicateDesktopEntries(desktopEntries)
}

func filterDuplicateDesktopEntries(desktopEntries []DesktopEntryWithPath) []DesktopEntryWithPath {
	desktopEntiesMap := map[string]DesktopEntryWithPath{}
	for _, desktopEntry := range desktopEntries {
		desktopEntiesMap[desktopEntry.entry.Name] = desktopEntry
	}

	uniqueDesktopEntries := []DesktopEntryWithPath{}
	for _, desktopEntry := range desktopEntiesMap {
		uniqueDesktopEntries = append(uniqueDesktopEntries, desktopEntry)
	}

	return uniqueDesktopEntries
}
