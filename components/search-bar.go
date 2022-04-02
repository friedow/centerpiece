package components

import (
	"github.com/gotk3/gotk3/gtk"
)

func SearchBarNew() *gtk.Entry {
	searchBar, _ := gtk.EntryNew()
	searchBar.SetPlaceholderText("Search or jump to...")

	return searchBar
}
