package main

import (
	"friedow/tucan-search/components"

	"github.com/gotk3/gotk3/gtk"
)

func App() *gtk.Box {
	verticalBox, _ := gtk.BoxNew(gtk.ORIENTATION_VERTICAL, 0)

	searchBar := components.SearchBarNew()
	verticalBox.Add(searchBar)

	optionList := components.OptionListNew()
	verticalBox.Add(optionList)

	return verticalBox
}
