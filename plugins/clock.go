package plugins

import "friedow/tucan-search/plugins/clock"

func newClockPluginOptions() []PluginOption {
	clockOptions := []PluginOption{}

	clockOptions = append(clockOptions, clock.NewTime())
	clockOptions = append(clockOptions, clock.NewDate())

	return clockOptions
}
