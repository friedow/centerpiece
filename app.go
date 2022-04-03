package main

import (
	"friedow/tucan-search/widgets"

	"github.com/gotk3/gotk3/gdk"
	"github.com/gotk3/gotk3/gtk"
)

func App() *gtk.Box {
	optionList := widgets.OptionListNew()
	searchBar := widgets.SearchBarNew(func(_ *gtk.Entry, event *gdk.Event) bool { return onKeyPress(optionList, event) })

	verticalBox, _ := gtk.BoxNew(gtk.ORIENTATION_VERTICAL, 0)
	verticalBox.Add(searchBar)
	verticalBox.Add(optionList)

	return verticalBox
}

// Handle keypress events manually for the option list
// and do not propate them to childs widgets
// to prevent the option list from picking up focus
func onKeyPress(optionList *gtk.ListBox, event *gdk.Event) bool {
	key := gdk.EventKeyNewFromEvent(event)

	if key.KeyVal() == gdk.KEY_Up || key.KeyVal() == gdk.KEY_Down || key.KeyVal() == gdk.KEY_Return {
		widgets.OnOptionListKeyPress(optionList, event)
		return true
	}

	return false
}
