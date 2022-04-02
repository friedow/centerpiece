package main

import (
	"friedow/tucan-search/components"

	"github.com/gotk3/gotk3/gdk"
	"github.com/gotk3/gotk3/gtk"
)

func App() *gtk.Box {
	verticalBox, _ := gtk.BoxNew(gtk.ORIENTATION_VERTICAL, 0)

	searchBar := components.SearchBarNew()
	verticalBox.Add(searchBar)

	optionList := components.OptionListNew()
	verticalBox.Add(optionList)

	verticalBox.Connect("key_press_event", func(_ *gtk.Box, event *gdk.Event) bool { return onKeyPress(optionList, event) })

	return verticalBox
}

// Handle keypress events manually for the option list
// and do not propate them to childs widgets
// to prevent the option list from picking up focus
func onKeyPress(optionList *gtk.ListBox, event *gdk.Event) bool {
	components.OnOptionListKeyPress(optionList, event)
	return true
}
