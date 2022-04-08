package widgets

import (
	"github.com/gotk3/gotk3/gtk"
)

type ProgressOptionComponent struct {
	*gtk.Box
	progress float64

	textContainer struct {
		*gtk.Box

		title  *gtk.Label
		action *gtk.Label
	}

	progressBar *gtk.ProgressBar
}

func NewProgressOptionComponent(title string, actionText string, progress float64) *ProgressOptionComponent {
	c := ProgressOptionComponent{}
	c.progress = progress

	c.textContainer.title, _ = gtk.LabelNew(title)
	c.textContainer.action, _ = gtk.LabelNew(actionText)

	c.textContainer.Box, _ = gtk.BoxNew(gtk.ORIENTATION_HORIZONTAL, 0)
	c.textContainer.PackStart(c.textContainer.title, false, false, 0)
	c.textContainer.PackEnd(c.textContainer.action, false, false, 0)

	c.progressBar, _ = gtk.ProgressBarNew()
	c.progressBar.SetFraction(progress)

	c.Box, _ = gtk.BoxNew(gtk.ORIENTATION_VERTICAL, 0)
	c.Add(c.textContainer)
	c.Add(c.progressBar)

	return &c

}

func (c *ProgressOptionComponent) Progress() float64 {
	return c.progress
}

func (c *ProgressOptionComponent) SetProgress(progress float64) {
	c.progress = progress
	c.update()
}

func (c *ProgressOptionComponent) update() {
	c.progressBar.SetFraction(c.progress)
}
