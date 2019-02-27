package isogram

import (
	"strings"
	"unicode"
)

// IsIsogram determines whether a given string is an isogram.
func IsIsogram(s string) bool {
	s = strings.ToLower(s)
	seen := make(map[rune]bool)
	for _, char := range s {
		if !unicode.IsLetter(char) {
			continue
		}
		if ok := seen[char]; ok {
			return false
		}
		seen[char] = true
	}
	return true
}
