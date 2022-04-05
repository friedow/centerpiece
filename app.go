package main

import (
	"friedow/tucan-search/widgets"

	"github.com/gotk3/gotk3/gdk"
	"github.com/gotk3/gotk3/gtk"
)

func App() *gtk.Box {
	scrolledWindow, _ := gtk.ScrolledWindowNew(nil, nil)
	optionList := widgets.OptionListNew()
	scrolledWindow.Add(optionList)
	scrolledWindow.SetMinContentHeight(700)

	searchBar := widgets.SearchBarNew(func(_ *gtk.Entry, event *gdk.Event) bool { return onKeyPress(optionList, event) })
	searchBar.Connect("changed", func() { onQueryChanged(optionList) })

	widgets.SetFilterFunction(optionList, searchBar)

	verticalBox, _ := gtk.BoxNew(gtk.ORIENTATION_VERTICAL, 0)
	verticalBox.Add(searchBar)
	verticalBox.Add(scrolledWindow)

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

func onQueryChanged(optionList *gtk.ListBox) {
	optionList.InvalidateFilter()
}
