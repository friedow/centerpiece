package options

import "github.com/diamondburned/gotk4/pkg/gtk/v4"

type TextOption struct {
	*gtk.Box

	titleLabel  *gtk.Label
	actionLabel *gtk.Label
}

func NewTextOption(title string, action string) *TextOption {
	this := TextOption{}

	this.titleLabel = gtk.NewLabel(title)
	this.actionLabel = gtk.NewLabel(action)

	this.Box = gtk.NewBox(gtk.OrientationHorizontal, 20)
	this.Append(this.titleLabel)
	this.Append(this.actionLabel)

	return &this
}

func (this TextOption) SetTitle(title string) {
	this.titleLabel.SetText(title)
}

func (this TextOption) SetAction(action string) {
	this.actionLabel.SetText(action)
}
