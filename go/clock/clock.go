package clock

import "fmt"

// Clock represents a clock, holding time.
type Clock struct {
	hour   int
	minute int
}

// New returns a new Clock.
func New(hours, minutes int) Clock {
	var minsToHrs int

	if minutes < 0 {
		for minutes < 0 {
			minutes += 60
			minsToHrs--
		}
	} else {
		minsToHrs = minutes / 60
	}
	minutes = minutes % 60

	hours += minsToHrs
	for hours < 0 {
		hours += 24
	}
	hours = hours % 24

	return Clock{
		hour:   hours,
		minute: minutes,
	}
}

// String satisfies the Stringer interface for Clock.
func (c Clock) String() string {
	var (
		hour   string
		minute string
	)

	if c.hour < 10 {
		hour = fmt.Sprintf("0%d", c.hour)
	} else {
		hour = fmt.Sprintf("%d", c.hour)
	}

	if c.minute < 10 {
		minute = fmt.Sprintf("0%d", c.minute)
	} else {
		minute = fmt.Sprintf("%d", c.minute)
	}

	return fmt.Sprintf("%s:%s", hour, minute)
}

// Add adds the given minutes to the current Clock.
func (c Clock) Add(minutes int) Clock {
	c.minute += minutes
	newHrs := c.minute / 60
	c.minute = c.minute % 60

	c.hour += newHrs
	c.hour = c.hour % 24
	return c
}

// Subtract subtracts the given minutes to the current Clock.
func (c Clock) Subtract(minutes int) Clock {
	var hrsToSubtract int
	c.minute -= minutes
	for c.minute < 0 {
		c.minute += 60
		hrsToSubtract++
	}
	c.hour -= hrsToSubtract
	for c.hour < 0 {
		c.hour += 24
	}
	return c
}
