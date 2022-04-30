package plugins

import (
	system_monitor "friedow/tucan-search/plugins/system-monitor"
)

func NewSystemMonitorPluginOptions() []PluginOption {
	systemStatistics := []PluginOption{}

	if system_monitor.DeviceHasBattery() {
		systemStatistics = append(systemStatistics, system_monitor.NewBatteryStatistic())
	}
	systemStatistics = append(systemStatistics, system_monitor.NewCpuStatistic())
	systemStatistics = append(systemStatistics, system_monitor.NewMemoryStatistic())

	return systemStatistics
}
