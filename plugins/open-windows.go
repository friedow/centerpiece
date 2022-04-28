package plugins

import (
	"encoding/json"
	"fmt"
	"friedow/tucan-search/components/options"
	"os/exec"
	"strings"
)

type I3MsgJsonPart struct {
	Id         int             `json:"id"`
	Name       string          `json:"name"`
	WindowType string          `json:"window_type"`
	Nodes      []I3MsgJsonPart `json:"nodes"`
}

func NewOpenWindowsPluginOptions() []PluginOption {
	i3MsgOutput, _ := exec.Command("i3-msg", "-t", "get_tree").Output()
	i3MsgJson := I3MsgJsonPart{}
	json.Unmarshal(i3MsgOutput, &i3MsgJson)
	windows := findWindows(i3MsgJson)

	pluginOptions := []PluginOption{}
	for _, window := range windows {
		pluginOptions = append(pluginOptions, window)
	}
	return pluginOptions
}

func findWindows(i3MsgJsonPart I3MsgJsonPart) []*OpenWindow {
	if i3MsgJsonPart.WindowType != "" {
		window := NewOpenWindow(i3MsgJsonPart.Id, i3MsgJsonPart.Name)
		return []*OpenWindow{window}
	}

	if i3MsgJsonPart.Nodes != nil {
		windows := []*OpenWindow{}
		for _, i3MsgJsonChild := range i3MsgJsonPart.Nodes {
			childWindows := findWindows(i3MsgJsonChild)

			windows = append(windows, childWindows...)
		}
		return windows
	}

	return []*OpenWindow{}
}

type OpenWindow struct {
	*options.TextOption

	id    int
	title string
}

var _ PluginOption = OpenWindow{}

func NewOpenWindow(id int, title string) *OpenWindow {
	this := OpenWindow{}

	this.TextOption = options.NewTextOption(title, "Enter to jump to")

	this.id = id
	this.title = title

	return &this
}

func (this OpenWindow) PluginName() string {
	return "Open Windows"
}

func (this OpenWindow) OnActivate() {
	focusWindowArgument := fmt.Sprintf("[con_id=%d] focus", this.id)
	exec.Command("i3-msg", focusWindowArgument).Run()
}

func (this OpenWindow) IsVisible(queryPart string) bool {
	return strings.Contains(strings.ToLower(this.title), queryPart)
}
