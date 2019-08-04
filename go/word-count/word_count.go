package wordcount

import (
	"strings"
	"unicode"
)

// Frequency maps a word to the number of times it appears.
type Frequency map[string]int

// WordCount outputs a Frequency for a given phrase.
func WordCount(phrase string) Frequency {
	freq := make(Frequency)

	split := func(r rune) bool {
		return !unicode.IsLetter(r) && !unicode.IsNumber(r) && !isApostrophe(r)
	}

	list := strings.FieldsFunc(phrase, split)
	for _, word := range list {
		w := strings.TrimFunc(strings.ToLower(word), isApostrophe)
		_, ok := freq[w]
		if !ok {
			freq[w] = 1
			continue
		}
		freq[w]++
	}

	return freq
}

func isApostrophe(r rune) bool {
	if string(r) == "'" {
		return true
	}
	return false
}
