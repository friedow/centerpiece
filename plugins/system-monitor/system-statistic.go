package system_monitor

import "strings"

type SystemStatistic struct {
	name string
}

func newSystemStatistic(name string) *SystemStatistic {
	this := SystemStatistic{}

	this.name = name

	return &this
}

func (this SystemStatistic) PluginName() string {
	return "System Monitor"
}

func (this SystemStatistic) OnActivate() {
	// do nothing
}

func (this SystemStatistic) IsVisible(queryPart string) bool {
	return strings.Contains(strings.ToLower(this.name), queryPart)
}
