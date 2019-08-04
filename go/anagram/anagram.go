package anagram

import "strings"

// Detect finds any anagrams of a given word, among a set of
// candidates.
func Detect(word string, candidates []string) []string {
	var anagrams []string

	wordCount := letterCounts(word)
	for _, candidate := range candidates {
		if strings.ToLower(word) == strings.ToLower(candidate) {
			continue
		}
		candidateCount := letterCounts(candidate)
		if areMapsEqual(wordCount, candidateCount) {
			anagrams = append(anagrams, candidate)
		}
	}

	return anagrams
}

func letterCounts(word string) map[string]int {
	m := make(map[string]int)
	normalized := strings.ToLower(word)

	for i := 0; i < len(normalized); i++ {
		m[string(normalized[i])] = strings.Count(normalized, string(normalized[i]))
	}

	return m
}

func areMapsEqual(m1, m2 map[string]int) bool {
	if len(m1) != len(m2) {
		return false
	}

	for k, v := range m1 {
		if m2[k] != v {
			return false
		}
	}

	return true
}
