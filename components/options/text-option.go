package options

import "github.com/diamondburned/gotk4/pkg/gtk/v4"

type TextOption struct {
	*gtk.CenterBox

	titleLabel  *gtk.Label
	actionLabel *gtk.Label
}

func NewTextOption(title string, action string) *TextOption {
	this := TextOption{}

	this.titleLabel = gtk.NewLabel(title)
	this.actionLabel = gtk.NewLabel(action)
	this.actionLabel.AddCSSClass("action")

	this.CenterBox = gtk.NewCenterBox()
	this.SetStartWidget(this.titleLabel)
	this.SetEndWidget(this.actionLabel)

	return &this
}

func (this TextOption) SetTitle(title string) {
	this.titleLabel.SetText(title)
}

func (this TextOption) SetAction(action string) {
	this.actionLabel.SetText(action)
}
