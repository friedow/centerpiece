package widgets

import (
	"github.com/gotk3/gotk3/gtk"
)

func SearchBarNew() *gtk.Entry {
	searchBar, _ := gtk.EntryNew()
	searchBar.SetPlaceholderText("Search or jump to...")
	// searchBar.Connect("key_press_event", onKeyPress)
	return searchBar
}
