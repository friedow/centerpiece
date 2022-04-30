package views

import (
	"friedow/tucan-search/components"

	"github.com/diamondburned/gotk4/pkg/gdk/v4"
	"github.com/diamondburned/gotk4/pkg/gtk/v4"
)

type SearchView struct {
	*gtk.Box

	searchBar  *components.SearchBar
	optionList *components.OptionList
}

func NewSearchView() *SearchView {
	this := SearchView{}

	this.searchBar = components.NewSearchBar()

	searchBarKeyEventController := gtk.NewEventControllerKey()
	searchBarKeyEventController.ConnectKeyPressed(func(keyVal uint, _ uint, _ gdk.ModifierType) bool {
		result := this.optionList.OnKeyPress(keyVal)
		this.searchBar.GrabFocus()
		return result
	})
	this.searchBar.AddController(searchBarKeyEventController)
	this.searchBar.ConnectActivate(func() { this.optionList.OnActivate() })
	this.searchBar.ConnectChanged(func() { this.optionList.FilterOptions(this.searchBar.Text()) })

	this.optionList = components.NewOptionList()

	scrolledWindow := gtk.NewScrolledWindow()
	scrolledWindow.SetMinContentHeight(700)
	scrolledWindow.SetChild(this.optionList)

	this.Box = gtk.NewBox(gtk.OrientationVertical, 0)
	this.Append(this.searchBar)
	this.Append(scrolledWindow)

	return &this
}
