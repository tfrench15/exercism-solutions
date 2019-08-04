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

	isApostrophe := func(ch string) bool {
		if ch == "'" {
			return true
		}
		return false
	}

	split := func(ch rune) bool {
		return !unicode.IsLetter(ch) && !unicode.IsNumber(ch) && !isApostrophe(string(ch))
	}

	list := strings.FieldsFunc(phrase, split)
	for _, word := range list {
		w := strings.ToLower(word)
		_, ok := freq[w]
		if !ok {
			freq[w] = 1
			continue
		}
		freq[w]++
	}

	return freq
}

func isApostrophe(word string) bool {
	for idx, r := range word {
		if string(r) == "'" {
			if !unicode.IsLetter(word[idx]-1) && !unicode.IsLetter(rune(word[idx]+1)) {
				return false
			}
		}
	}
	return true
}
