package components

import "github.com/diamondburned/gotk4/pkg/gtk/v4"

type SearchBar struct {
	*gtk.Entry
}

func NewSearchBar() *SearchBar {
	this := SearchBar{}

	this.Entry = gtk.NewEntry()
	this.SetPlaceholderText("Search or jump to...")
	this.AddCSSClass("search-bar")

	return &this
}
