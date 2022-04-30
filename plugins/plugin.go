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
	pluginOptions = append(pluginOptions, newSystemMonitorPluginOptions()...)
	pluginOptions = append(pluginOptions, newOpenWindowsPluginOptions()...)
	pluginOptions = append(pluginOptions, newApplicationsPluginOptions()...)
	pluginOptions = append(pluginOptions, newGitRepositoriesPluginOptions()...)
	return pluginOptions
}
