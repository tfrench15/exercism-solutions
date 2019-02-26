package hamming

import "errors"

// Distance takes two strings representing DNA strands and
// returns the Hamming error between them.
func Distance(a, b string) (int, error) {
	if len(a) != len(b) {
		return 0, errors.New("lengths of each strand are different")
	}
	if len(a) == 0 && len(b) == 0 {
		return 0, nil
	}

	hamming := 0
	for i := 0; i < len(a); i++ {
		if a[i] != b[i] {
			hamming++
		}
	}
	return hamming, nil
}
