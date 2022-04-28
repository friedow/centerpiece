package main

import (
	"friedow/tucan-search/views"
	"os"

	"github.com/diamondburned/gotk4/pkg/gtk/v4"
)

func main() {
	app := gtk.NewApplication("com.github.friedow.tucan-search", 0)
	app.ConnectActivate(func() { activate(app) })

	code := app.Run(os.Args)
	if code > 0 {
		os.Exit(code)
	}
}

func activate(app *gtk.Application) {
	window := gtk.NewApplicationWindow(app)
	window.SetTitle("Tucan Search")
	window.SetDefaultSize(800, 600)
	window.SetModal(true)

	searchView := views.NewSearchView()
	window.SetChild(searchView.Box)

	window.Show()
}
