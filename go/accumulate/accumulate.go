package accumulate

// Accumulate operates on each word in the given string with
// the given function, and returns a new slice.
func Accumulate(words []string, fn func(string) string) []string {
	var ret []string
	for _, word := range words {
		r := fn(word)
		ret = append(ret, r)
	}
	return ret
}
