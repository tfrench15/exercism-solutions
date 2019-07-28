package erratum

// Use is the function required for this exercise.
func Use(o ResourceOpener, input string) error {
	resource, err := o()
	defer resource.Close()
	switch err {
	case nil:
		resource.Frob(input)
		return nil
	case TransientError{err}:
		return Use(o, input)
	default:
		return err
	}
}
