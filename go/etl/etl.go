package etl

import "strings"

type given map[int][]string
type expectation map[string]int

// Transform takes an old scorecard and converts it into a new format.
func Transform(scorecard given) expectation {
	newScorecard := make(expectation)

	for k, v := range scorecard {
		for i := 0; i < len(v); i++ {
			newScorecard[strings.ToLower(v[i])] = k
		}
	}

	return newScorecard
}
