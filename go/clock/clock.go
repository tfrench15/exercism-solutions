package clock

// Clock represents a clock, holding time.
type Clock struct {
	hour   int
	minute int
}

// New returns a new Clock.
func New(hour, minute int) Clock {
	return Clock{}
}

// String satisfies the Stringer interface for Clock.
func (c *Clock) String() string {
	return ""
}

// Add adds the given minutes to the current Clock.
func (c Clock) Add(minutes int) Clock {
	return c
}

// Subtract subtracts the given minutes to the current Clock.
func (c Clock) Subtract(minutes int) Clock {
	return c
}
