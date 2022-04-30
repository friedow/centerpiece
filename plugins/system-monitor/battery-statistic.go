package system_monitor

import (
	"fmt"
	"friedow/tucan-search/components/options"

	"github.com/diamondburned/gotk4/pkg/glib/v2"
	"github.com/distatus/battery"
)

type BatteryStatistic struct {
	*options.ProgressOption
	*SystemStatistic
}

func DeviceHasBattery() bool {
	return firstBattery() != nil
}

func firstBattery() *battery.Battery {
	batteries, err := battery.GetAll()

	if len(batteries) <= 0 || err != nil {
		return nil
	}

	return batteries[0]
}

func NewBatteryStatistic() *BatteryStatistic {
	this := BatteryStatistic{}

	this.SystemStatistic = newSystemStatistic("Battery")
	this.ProgressOption = options.NewProgressOption(this.title(), "", this.chargeAsDecimalFraction())

	glib.TimeoutAdd(5000, func() bool {
		this.update()
		return true
	})

	return &this
}

func (this BatteryStatistic) update() {
	this.SetTitle(this.title())
	this.SetProgress(this.chargeAsDecimalFraction())
}

func (this BatteryStatistic) title() string {
	return fmt.Sprintf("%s %d%% – %s", this.name, int(this.chargeAsPercent()), this.state())
}

func (this BatteryStatistic) chargeAsDecimalFraction() float64 {
	battery := firstBattery()
	if battery == nil {
		return 0
	}

	return battery.Current / battery.Full
}

func (this BatteryStatistic) chargeAsPercent() float64 {
	return this.chargeAsDecimalFraction() * 100
}

func (this BatteryStatistic) state() string {
	battery := firstBattery()
	if battery == nil {
		return ""
	}
	return battery.State.String()
}
