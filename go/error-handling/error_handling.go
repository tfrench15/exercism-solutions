package erratum

// Use is the function required for this exercise.
func Use(o ResourceOpener, input string) error {
	resource, err := o()
	if err != nil {
		if e, ok := err.(TransientError) {
			return o()
		}
	}
}
