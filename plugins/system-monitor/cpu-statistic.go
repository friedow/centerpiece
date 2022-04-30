package system_monitor

// #include <stdio.h>
// #include <time.h>
import "C"

import (
	"fmt"
	"friedow/tucan-search/components/options"
	"time"

	"github.com/diamondburned/gotk4/pkg/glib/v2"
)

type CpuStatistic struct {
	*options.ProgressOption
	*SystemStatistic

	startTime  time.Time
	startTicks float64

	cpuUsageInPercent float64
}

func NewCpuStatistic() *CpuStatistic {
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
