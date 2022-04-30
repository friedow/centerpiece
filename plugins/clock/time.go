package clock

import (
	"fmt"
	"friedow/tucan-search/components/options"
	"time"

	"github.com/diamondburned/gotk4/pkg/glib/v2"
)

type Time struct {
	*options.TextOption
	*clockOption
}

func NewTime() *Time {
	this := Time{}

	this.clockOption = newClockOption("Time")
	this.TextOption = options.NewTextOption(this.title(), "")

	glib.TimeoutAdd(1000, func() bool {
		this.update()
		return true
	})

	return &this
}

func (this Time) update() {
	this.SetTitle(this.title())
}

func (this Time) title() string {
	hour, minute, second := time.Now().Clock()
	return fmt.Sprintf("%d:%d:%d", hour, minute, second)
}
