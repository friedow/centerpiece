package widgets

import (
	"github.com/gotk3/gotk3/gtk"
)

func OptionWidgetNew(title string, actionText string) *gtk.Box {
	hbox, _ := gtk.BoxNew(gtk.ORIENTATION_HORIZONTAL, 10)

	titleLabel, _ := gtk.LabelNew(title)
	hbox.PackStart(titleLabel, false, false, 10)

	actionLabel, _ := gtk.LabelNew(actionText)
	hbox.PackEnd(actionLabel, false, false, 10)

	return hbox
}
