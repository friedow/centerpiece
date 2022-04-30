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

	this.UpdateMemoryUsage()
	this.ProgressOption = options.NewProgressOption(this.Title(), "", this.MemoryUsageAsDecimalFraction())

	glib.TimeoutAdd(3000, func() bool {
		this.UpdateMemoryUsage()
		this.UpdateWidget()
		return true
	})

	return &this
}

func (this *MemoryStatistic) UpdateMemoryUsage() {
	v, _ := mem.VirtualMemory()
	this.memoryUsageInPercent = v.UsedPercent
}

func (this MemoryStatistic) UpdateWidget() {
	this.SetTitle(this.Title())
	this.SetProgress(this.MemoryUsageAsDecimalFraction())
}

func (this MemoryStatistic) Title() string {
	return fmt.Sprintf("%s %d%%", this.name, int(this.MemoryUsageInPercent()))
}

func (this MemoryStatistic) MemoryUsageAsDecimalFraction() float64 {
	return this.MemoryUsageInPercent() * 0.01
}

func (this MemoryStatistic) MemoryUsageInPercent() float64 {
	return this.memoryUsageInPercent
}
