package plugins

import (
	"github.com/gotk3/gotk3/gdk"
	"github.com/gotk3/gotk3/gtk"
)

type Plugin interface {
	GetName() string
	GetOptions() []*gtk.Box
	OnKeyPressed(option *gtk.Box, key *gdk.Event)
}

func Plugins() []Plugin {
	return []Plugin{
		GitRepositoriesPlugin{},
	}
}
