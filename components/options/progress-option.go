package options

import "github.com/diamondburned/gotk4/pkg/gtk/v4"

type ProgressOption struct {
	*gtk.Box

	textOption  *TextOption
	progressBar *gtk.ProgressBar
}

func NewProgressOption(title string, action string, progress float64) *ProgressOption {
	this := ProgressOption{}

	this.textOption = NewTextOption(title, action)

	this.progressBar = gtk.NewProgressBar()
	this.progressBar.SetPulseStep(progress)

	this.Box = gtk.NewBox(gtk.OrientationVertical, 0)
	this.Append(this.textOption)
	this.Append(this.progressBar)

	return &this
}

func (this ProgressOption) SetTitle(title string) {
	this.textOption.SetTitle(title)
}

func (this ProgressOption) SetAction(action string) {
	this.textOption.SetAction(action)
}

func (this ProgressOption) SetProgress(progress float64) {
	this.progressBar.SetFraction(progress)
}
