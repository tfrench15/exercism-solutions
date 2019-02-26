package acronym

import (
	"strings"
	"unicode"
)

// Abbreviate returns an acronym for the given string.
func Abbreviate(s string) string {
	acronym := ""
	words := strings.Split(s, " ")
	for _, word := range words {
		if unicode.IsLetter(rune(word[0])) {
			acronym += strings.ToUpper(string(word[0]))
			if hasHyphen(word) {
				idx := strings.Index(word, "-")
				acronym += strings.ToUpper(string(word[idx+1]))
			}
		}
	}
	return acronym
}

func hasHyphen(s string) bool {
	for i := 0; i < len(s); i++ {
		if string(s[i]) == "-" {
			return true
		}
	}
	return false
}
