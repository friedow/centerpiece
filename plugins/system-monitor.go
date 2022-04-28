package plugins

import (
	"fmt"
	"friedow/tucan-search/components/options"
	"strings"

	"github.com/diamondburned/gotk4/pkg/glib/v2"
	"github.com/distatus/battery"
)

func NewSystemMonitorPluginOptions() []PluginOption {
	systemStatistics := []PluginOption{}

	if firstBattery() != nil {
		systemStatistics = append(systemStatistics, newBatteryStatistic())
	}

	return systemStatistics
}

func firstBattery() *battery.Battery {
	batteries, err := battery.GetAll()

	if len(batteries) <= 0 || err != nil {
		return nil
	}

	return batteries[0]
}

type BatteryStatistic struct {
	*SystemStatistic
}

func newBatteryStatistic() *BatteryStatistic {
	this := BatteryStatistic{}

	this.SystemStatistic = NewSystemStatistic("Battery", this.CurrentValue(), this.MaximumCapacity(), "mWh")

	glib.TimeoutAdd(1000, func() bool {
		this.SetCurrentValue(this.CurrentValue())
		this.SetTitle(this.Title())
		return true
	})

	return &this
}

// func (this BatteryStatistic) SetCurrentValue(currentValue float64) {
// 	this.currentValue = currentValue
// 	this.SetTitle(this.Title())
// 	this.SetProgress(this.currentProgress() * 0.01)
// }

func (this BatteryStatistic) Title() string {
	return fmt.Sprintf("%s %d%% %s", this.name, int(this.currentProgress()), this.State())
}

func (this BatteryStatistic) CurrentValue() float64 {
	battery := firstBattery()
	if battery == nil {
		return 0
	}
	return battery.Current
}

func (this BatteryStatistic) MaximumCapacity() float64 {
	battery := firstBattery()
	if battery == nil {
		return 0
	}
	return battery.Full
}

func (this BatteryStatistic) State() string {
	battery := firstBattery()
	if battery == nil {
		return ""
	}
	return battery.State.String()
}

type SystemStatistic struct {
	*options.ProgressOption

	name            string
	currentValue    float64
	maximumCapacity float64
	unit            string
}

var _ PluginOption = SystemStatistic{}

func NewSystemStatistic(name string, currentValue float64, maximumCapacity float64, unit string) *SystemStatistic {
	this := SystemStatistic{}

	this.name = name
	this.maximumCapacity = maximumCapacity
	this.unit = unit

	this.ProgressOption = options.NewProgressOption(this.Title(), "", 0)
	this.SetCurrentValue(currentValue)

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

func (this SystemStatistic) SetCurrentValue(currentValue float64) {
	this.currentValue = currentValue
	this.SetTitle(this.Title())
	this.SetProgress(this.currentProgress() * 0.01)
}

func (this SystemStatistic) Title() string {
	return fmt.Sprintf("%s %d%% (%d / %d %s)", this.name, int(this.currentProgress()), int(this.currentValue), int(this.maximumCapacity), this.unit)
}

func (this SystemStatistic) currentProgress() float64 {
	return this.currentValue * 100 / this.maximumCapacity
}
