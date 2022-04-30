package plugins

// #include <stdio.h>
// #include <time.h>
import "C"

import (
	"fmt"
	"friedow/tucan-search/components/options"
	"strings"
	"time"

	"github.com/diamondburned/gotk4/pkg/glib/v2"
	"github.com/distatus/battery"
)

func NewSystemMonitorPluginOptions() []PluginOption {
	systemStatistics := []PluginOption{}

	if firstBattery() != nil {
		systemStatistics = append(systemStatistics, newBatteryStatistic())
	}
	systemStatistics = append(systemStatistics, newCpuStatistic())

	return systemStatistics
}

type BatteryStatistic struct {
	*options.ProgressOption
	*SystemStatistic
}

var _ PluginOption = BatteryStatistic{}

func firstBattery() *battery.Battery {
	batteries, err := battery.GetAll()

	if len(batteries) <= 0 || err != nil {
		return nil
	}

	return batteries[0]
}

func newBatteryStatistic() *BatteryStatistic {
	this := BatteryStatistic{}

	this.SystemStatistic = newSystemStatistic("Battery")
	this.ProgressOption = options.NewProgressOption(this.Title(), "", this.ChargeInDecimalFraction())

	glib.TimeoutAdd(5000, func() bool {
		this.Update()
		return true
	})

	return &this
}

func (this BatteryStatistic) Update() {
	this.SetTitle(this.Title())
	this.SetProgress(this.ChargeInDecimalFraction())
}

func (this BatteryStatistic) Title() string {
	return fmt.Sprintf("%s %d%% %s", this.name, int(this.ChargeInPercent()), this.State())
}

func (this BatteryStatistic) ChargeInDecimalFraction() float64 {
	battery := firstBattery()
	if battery == nil {
		return 0
	}

	return battery.Current / battery.Full
}

func (this BatteryStatistic) ChargeInPercent() float64 {
	return this.ChargeInDecimalFraction() * 100
}

func (this BatteryStatistic) State() string {
	battery := firstBattery()
	if battery == nil {
		return ""
	}
	return battery.State.String()
}

type CpuStatistic struct {
	*options.ProgressOption
	*SystemStatistic

	startTime  time.Time
	startTicks float64

	cpuUsageInPercent float64
}

var _ PluginOption = CpuStatistic{}

func newCpuStatistic() *CpuStatistic {
	this := CpuStatistic{}

	this.SystemStatistic = newSystemStatistic("CPU")

	this.startTime = time.Now()
	this.startTicks = float64(C.clock())
	this.UpdateCpuUsage()

	this.ProgressOption = options.NewProgressOption(this.Title(), "", this.CpuUsageInDecimalFraction())

	glib.TimeoutAdd(3000, func() bool {
		this.UpdateCpuUsage()
		this.UpdateWidget()
		return true
	})

	return &this
}

func (this *CpuStatistic) UpdateCpuUsage() {
	clockSeconds := (float64(C.clock()) - this.startTicks) / float64(C.CLOCKS_PER_SEC)
	realSeconds := time.Since(this.startTime).Seconds()
	this.cpuUsageInPercent = (clockSeconds / realSeconds * 100)

	this.startTime = time.Now()
	this.startTicks = float64(C.clock())
}

func (this CpuStatistic) UpdateWidget() {
	this.SetTitle(this.Title())
	this.SetProgress(this.CpuUsageInDecimalFraction())
}

func (this CpuStatistic) Title() string {
	return fmt.Sprintf("%s %d%%", this.name, int(this.CpuUsageInPercent()))
}

func (this CpuStatistic) CpuUsageInDecimalFraction() float64 {
	return this.CpuUsageInPercent() * 0.01
}

func (this CpuStatistic) CpuUsageInPercent() float64 {
	return this.cpuUsageInPercent
}

type SystemStatistic struct {
	name string
}

func newSystemStatistic(name string) *SystemStatistic {
	this := SystemStatistic{}

	this.name = name

	return &this
}

func (this SystemStatistic) PluginName() string {
	return "System Monitor"
}

func (this SystemStatistic) OnActivate() {
	// do nothing
}

func (this SystemStatistic) IsVisible(queryPart string) bool {
	return strings.Contains(strings.ToLower(this.name), queryPart)
}
