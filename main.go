package main

import (
	_ "embed"
	"friedow/tucan-search/views"
	"log"
	"os"
	"strings"

	"github.com/diamondburned/gotk4/pkg/gdk/v4"
	"github.com/diamondburned/gotk4/pkg/gtk/v4"
)

//go:embed style.css
var styleCSS string

func main() {
	app := gtk.NewApplication("com.github.friedow.tucan-search", 0)
	app.ConnectActivate(func() { activate(app) })

	code := app.Run(os.Args)
	if code > 0 {
		os.Exit(code)
	}
}

func activate(app *gtk.Application) {
	gtk.StyleContextAddProviderForDisplay(
		gdk.DisplayGetDefault(), loadCSS(styleCSS),
		gtk.STYLE_PROVIDER_PRIORITY_APPLICATION,
	)

	window := gtk.NewApplicationWindow(app)
	window.SetTitle("Tucan Search")
	window.SetDefaultSize(800, 600)
	window.SetModal(true)

	searchView := views.NewSearchView()
	window.SetChild(searchView.Box)

	window.Show()
}

func loadCSS(content string) *gtk.CSSProvider {
	prov := gtk.NewCSSProvider()
	prov.ConnectParsingError(func(sec *gtk.CSSSection, err error) {
		// Optional line parsing routine.
		loc := sec.StartLocation()
		lines := strings.Split(content, "\n")
		log.Printf("CSS error (%v) at line: %q", err, lines[loc.Lines()])
	})
	prov.LoadFromData(content)
	return prov
}
