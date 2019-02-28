package reverse

import (
	"unicode/utf8"
)

// String reverses the provided string.
func String(s string) string {
	var rev string
	b := []byte(s)
	for len(b) > 0 {
		r, size := utf8.DecodeLastRune(b)
		rev += string(r)
		b = b[:len(b)-size]
	}
	return rev
}
