package pangram

import "strings"

var alphabet = "abcdefghijklmnopqrstuvwxyz"

// IsPangram determines whether the given sentence contains every letter of the alphabet.
func IsPangram(sentence string) bool {
	// initialize seen map
	seen := make(map[string]bool)
	for _, ch := range alphabet {
		seen[string(ch)] = false
	}

	// update map if letter is seen
	for _, ch := range sentence {
		_, ok := seen[strings.ToLower(ch)]
		if !ok {
			continue
		}
		seen[strings.ToLower(ch)] = true
	}

	// check if any haven't been seen; if so, the sentence is not a pangram
	for _, value := range seen {
		if !value {
			return false
		}
	}

	return true
}
