package luhn

import (
	"strconv"
	"strings"
)

// Valid determines whether a given string is valid
// per the Luhn forumla.
func Valid(s string) bool {
	if ok := isValidLengthAndCharacters(s); !ok {
		return false
	}

	num, _ := strconv.Atoi(s) // already checked strconv err in helper func.

	return true
}

func isValidLengthAndCharacters(s string) bool {
	s = strings.TrimSpace(s)
	if len(s) <= 1 {
		return false
	}
	_, err := strconv.Atoi(s)
	if err != nil {
		return false
	}
	return true
}
