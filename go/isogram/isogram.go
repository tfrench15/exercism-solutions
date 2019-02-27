package isogram

import (
	"strings"
	"unicode"
)

// IsIsogram determines whether a given string is an isogram.
func IsIsogram(s string) bool {
	s = strings.ToLower(s)
	seen := make(map[string]bool)
	for i := 0; i < len(s); i++ {
		char := string(s[i])
		if !unicode.IsLetter(rune(char)) {
			continue
		}
		if ok := seen[char]; ok {
			return false
		}
		seen[char] = true
	}
	return true
}
