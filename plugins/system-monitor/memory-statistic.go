package system_monitor

import (
	"fmt"
	"friedow/tucan-search/components/options"

	"github.com/diamondburned/gotk4/pkg/glib/v2"
	"github.com/shirou/gopsutil/mem"
)

type MemoryStatistic struct {
	*options.ProgressOption
	*SystemStatistic

	memoryUsageInPercent float64
}

func NewMemoryStatistic() *MemoryStatistic {
	this := MemoryStatistic{}

	this.SystemStatistic = newSystemStatistic("Memory")

	this.updateMemoryUsage()
	this.ProgressOption = options.NewProgressOption(this.title(), "", this.memoryUsageAsDecimalFraction())

	glib.TimeoutAdd(1000, func() bool {
		this.updateMemoryUsage()
		this.updateWidget()
		return true
	})

	return &this
}

func (this *MemoryStatistic) updateMemoryUsage() {
	v, _ := mem.VirtualMemory()
	this.memoryUsageInPercent = v.UsedPercent
}

func (this MemoryStatistic) updateWidget() {
	this.SetTitle(this.title())
	this.SetProgress(this.memoryUsageAsDecimalFraction())
}

func (this MemoryStatistic) title() string {
	return fmt.Sprintf("%s %d%%", this.name, int(this.memoryUsageAsPercent()))
}

func (this MemoryStatistic) memoryUsageAsDecimalFraction() float64 {
	return this.memoryUsageAsPercent() * 0.01
}

func (this MemoryStatistic) memoryUsageAsPercent() float64 {
	return this.memoryUsageInPercent
}
