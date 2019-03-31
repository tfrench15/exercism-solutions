package flatten

// Flatten takes a nested slice and returns a flattened
// version of the slice.
func Flatten(slc interface{}) []interface{} {
	ret := []interface{}{}

	switch elem := slc.(type) {
	case []interface{}:
		for _, v := range elem {
			ret = append(ret, Flatten(v)...)
		}
	case int:
		ret = append(ret, elem)
	}
	return ret
}
