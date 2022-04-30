package options

import (
	"github.com/diamondburned/gotk4/pkg/gtk/v4"
	"github.com/diamondburned/gotk4/pkg/pango"
)

type TextOption struct {
	*gtk.CenterBox

	titleLabel  *gtk.Label
	actionLabel *gtk.Label
}

func NewTextOption(title string, action string) *TextOption {
	this := TextOption{}

	this.titleLabel = gtk.NewLabel(title)
	this.titleLabel.SetMaxWidthChars(1)
	this.titleLabel.SetEllipsize(pango.EllipsizeEnd)
	this.titleLabel.SetHExpand(true)
	this.titleLabel.SetXAlign(0)
	this.titleLabel.AddCSSClass("title")

	this.actionLabel = gtk.NewLabel(action)
	this.actionLabel.AddCSSClass("action")

	this.CenterBox = gtk.NewCenterBox()
	this.SetStartWidget(this.titleLabel)
	this.SetEndWidget(this.actionLabel)
	this.AddCSSClass("text-option")

	return &this
}

func (this TextOption) SetTitle(title string) {
	this.titleLabel.SetText(title)
}

func (this TextOption) SetAction(action string) {
	this.actionLabel.SetText(action)
}
