package plugins

import (
	"encoding/json"
	"fmt"
	"friedow/tucan-search/models"
	"os/exec"
)

type OpenWindowsPlugin struct{}

func (m OpenWindowsPlugin) GetName() string {
	return "Open Windows"
}

func (m OpenWindowsPlugin) GetOptionModels() []models.OptionModel {
	openWindows := getOpenWindows()

	options := []models.OptionModel{}
	for _, openWindow := range openWindows {

		option := models.OptionModel{
			PluginName: m.GetName(),
			Title:      openWindow.Name,
			ActionText: "enter to switch to",
			Data:       openWindow,
		}
		options = append(options, option)
	}

	return options
}

func (m OpenWindowsPlugin) OnActivate(optionModel models.OptionModel) {
	window := optionModel.Data.(I3MsgJsonPart)
	focusWindowArgument := fmt.Sprintf("[con_id=%d] focus", window.Id)
	exec.Command("i3-msg", focusWindowArgument).Run()
}

type I3MsgJsonPart struct {
	Id         int             `json:"id"`
	Name       string          `json:"name"`
	WindowType string          `json:"window_type"`
	Nodes      []I3MsgJsonPart `json:"nodes"`
}

func getOpenWindows() []I3MsgJsonPart {
	i3MsgOutput, _ := exec.Command("i3-msg", "-t", "get_tree").Output()
	i3MsgJson := I3MsgJsonPart{}
	json.Unmarshal(i3MsgOutput, &i3MsgJson)

	return findWindows(i3MsgJson)
}

func findWindows(i3MsgJsonPart I3MsgJsonPart) []I3MsgJsonPart {
	if i3MsgJsonPart.WindowType != "" {
		return []I3MsgJsonPart{i3MsgJsonPart}
	}

	if i3MsgJsonPart.Nodes != nil {
		windows := []I3MsgJsonPart{}
		for _, i3MsgJsonChild := range i3MsgJsonPart.Nodes {
			childWindows := findWindows(i3MsgJsonChild)

			windows = append(windows, childWindows...)
		}
		return windows
	}

	return []I3MsgJsonPart{}
}
