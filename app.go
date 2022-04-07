package main

import (
	"friedow/tucan-search/widgets"

	"github.com/gotk3/gotk3/gdk"
	"github.com/gotk3/gotk3/gtk"
)

func App() *gtk.Box {
	searchBar := widgets.SearchBarNew()

	scrolledWindow, _ := gtk.ScrolledWindowNew(nil, nil)

	optionList := widgets.OptionListNew()
	scrolledWindow.Add(optionList)
	scrolledWindow.SetMinContentHeight(700)

	searchBar.Connect("key_press_event", func(_ *gtk.Entry, event *gdk.Event) bool { return onKeyPress(searchBar, optionList, event) })
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
func onKeyPress(searchBar *gtk.Entry, optionList *gtk.ListBox, event *gdk.Event) bool {
	key := gdk.EventKeyNewFromEvent(event)

	if key.KeyVal() == gdk.KEY_Up || key.KeyVal() == gdk.KEY_Down {
		selectedRow := optionList.GetSelectedRow()
		selectedRow.GrabFocus()
		optionList.Event(event)
		searchBar.GrabFocus()
		return true
	}

	if key.KeyVal() == gdk.KEY_Return {
		selectedRow := optionList.GetSelectedRow()
		optionInterface, _ := selectedRow.GetChild()
		option := optionInterface.ToWidget()
		option.Event(event)
		return true
	}

	return false
}

func onQueryChanged(optionList *gtk.ListBox) {
	optionList.InvalidateFilter()
}
