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
	return fmt.Sprintf("%s %d%% – %s", this.name, int(this.ChargeInPercent()), this.State())
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
