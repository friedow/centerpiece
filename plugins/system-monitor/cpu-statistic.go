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

	cpuUsagePercent float64
}

func NewCpuStatistic() *CpuStatistic {
	this := CpuStatistic{}

	this.SystemStatistic = newSystemStatistic("CPU")

	this.startTime = time.Now()
	this.startTicks = float64(C.clock())
	this.updateCpuUsage()

	this.ProgressOption = options.NewProgressOption(this.title(), "", this.cpuUsageAsDecimalFraction())

	glib.TimeoutAdd(1000, func() bool {
		this.updateCpuUsage()
		this.updateWidget()
		return true
	})

	return &this
}

func (this *CpuStatistic) updateCpuUsage() {
	clockSeconds := (float64(C.clock()) - this.startTicks) / float64(C.CLOCKS_PER_SEC)
	realSeconds := time.Since(this.startTime).Seconds()
	this.cpuUsagePercent = (clockSeconds / realSeconds * 100)

	this.startTime = time.Now()
	this.startTicks = float64(C.clock())
}

func (this CpuStatistic) updateWidget() {
	this.SetTitle(this.title())
	this.SetProgress(this.cpuUsageAsDecimalFraction())
}

func (this CpuStatistic) title() string {
	return fmt.Sprintf("%s %d%%", this.name, int(this.cpuUsageAsPercent()))
}

func (this CpuStatistic) cpuUsageAsDecimalFraction() float64 {
	return this.cpuUsageAsPercent() * 0.01
}

func (this CpuStatistic) cpuUsageAsPercent() float64 {
	return this.cpuUsagePercent
}
