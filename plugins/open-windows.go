package plugins

import (
	"encoding/json"
	"fmt"
	"friedow/tucan-search/components/options"
	"os/exec"
	"strings"
)

type i3MsgJsonPart struct {
	Id         int             `json:"id"`
	Name       string          `json:"name"`
	WindowType string          `json:"window_type"`
	Nodes      []i3MsgJsonPart `json:"nodes"`
}

func newOpenWindowsPluginOptions() []PluginOption {
	i3MsgOutput, _ := exec.Command("i3-msg", "-t", "get_tree").Output()
	i3MsgJson := i3MsgJsonPart{}
	json.Unmarshal(i3MsgOutput, &i3MsgJson)
	windows := findWindows(i3MsgJson)

	pluginOptions := []PluginOption{}
	for _, window := range windows {
		pluginOptions = append(pluginOptions, window)
	}
	return pluginOptions
}

func findWindows(i3MsgJson i3MsgJsonPart) []*openWindow {
	if i3MsgJson.WindowType != "" {
		window := newOpenWindow(i3MsgJson.Id, i3MsgJson.Name)
		return []*openWindow{window}
	}

	if i3MsgJson.Nodes != nil {
		windows := []*openWindow{}
		for _, i3MsgJsonChild := range i3MsgJson.Nodes {
			childWindows := findWindows(i3MsgJsonChild)

			windows = append(windows, childWindows...)
		}
		return windows
	}

	return []*openWindow{}
}

type openWindow struct {
	*options.TextOption

	id    int
	title string
}

var _ PluginOption = openWindow{}

func newOpenWindow(id int, title string) *openWindow {
	this := openWindow{}

	this.TextOption = options.NewTextOption(title, "Enter to jump to")

	this.id = id
	this.title = title

	return &this
}

func (this openWindow) PluginName() string {
	return "Open Windows"
}

func (this openWindow) OnActivate() {
	focusWindowArgument := fmt.Sprintf("[con_id=%d] focus", this.id)
	exec.Command("i3-msg", focusWindowArgument).Run()
}

func (this openWindow) IsVisible(queryPart string) bool {
	return strings.Contains(strings.ToLower(this.title), queryPart)
}
