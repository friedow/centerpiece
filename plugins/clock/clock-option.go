package clock

import "strings"

type clockOption struct {
	name string
}

func newClockOption(name string) *clockOption {
	this := clockOption{}

	this.name = name

	return &this
}

func (this clockOption) PluginName() string {
	return "Clock"
}

func (this clockOption) OnActivate() {
	// do nothing
}

func (this clockOption) IsVisible(queryPart string) bool {
	return strings.Contains(strings.ToLower(this.name), queryPart)
}
