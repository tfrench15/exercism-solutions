package gigasecond

import (
	"time"
)

// AddGigasecond takes a time.Time representing the date of
// someone's birth, and returns a time.Time representing the
// moment they've lived one gigasecond.
func AddGigasecond(t time.Time) time.Time {
	seconds := 1000000000 % 60
	minutes := 1000000000 / 60
	hours := minutes / 60
	minutes = minutes % 60
	days := hours / 24
	hours = hours % 24

	t = t.AddDate(0, 0, days)
	t = t.Add(time.Second * time.Duration(seconds))
	t = t.Add(time.Minute * time.Duration(minutes))
	t = t.Add(time.Hour * time.Duration(hours))
	return t
}
