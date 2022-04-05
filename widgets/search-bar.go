package widgets

import (
	"github.com/gotk3/gotk3/gdk"
	"github.com/gotk3/gotk3/gtk"
)

func SearchBarNew(onKeyPress func(Entry *gtk.Entry, event *gdk.Event) bool) *gtk.Entry {
	searchBar, _ := gtk.EntryNew()
	searchBar.SetPlaceholderText("Search or jump to...")
	return searchBar
}
