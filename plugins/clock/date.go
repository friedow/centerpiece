package clock

import (
	"fmt"
	"friedow/tucan-search/components/options"
	"time"

	"github.com/diamondburned/gotk4/pkg/glib/v2"
)

type Date struct {
	*options.TextOption
	*clockOption
}

func NewDate() *Date {
	this := Date{}

	this.clockOption = newClockOption("Date")
	this.TextOption = options.NewTextOption(this.title(), "")

	glib.TimeoutAdd(1000, func() bool {
		this.update()
		return true
	})

	return &this
}

func (this Date) update() {
	this.SetTitle(this.title())
}

func (this Date) title() string {
	currentTime := time.Now()
	year, month, day := currentTime.Date()
	weekday := currentTime.Weekday()

	return fmt.Sprintf("%s, %d. %s %d", weekday, day, month.String(), year)
}
