package plugins

import "github.com/diamondburned/gotk4/pkg/gtk/v4"

type PluginOption interface {
	gtk.Widgetter

	OnActivate()
	PluginName() string
	IsVisible(queryPart string) bool
}

func PluginOptions() []PluginOption {
	pluginOptions := []PluginOption{}
	pluginOptions = append(pluginOptions, NewSystemMonitorPluginOptions()...)
	pluginOptions = append(pluginOptions, NewOpenWindowsPluginOptions()...)
	pluginOptions = append(pluginOptions, NewApplicationsPluginOptions()...)
	pluginOptions = append(pluginOptions, NewGitRepositoriesPluginOptions()...)
	return pluginOptions
}
