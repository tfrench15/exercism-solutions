package dna

import "errors"

// Histogram is a mapping from nucleotide to its count in given DNA.
type Histogram map[rune]int

// DNA represents a strand of DNA.
type DNA string

// Counts generates a histogram of valid nucleotides in the given DNA.
// Returns an error if d contains an invalid nucleotide.
func (d DNA) Counts() (Histogram, error) {
	if d == "" {
		return Histogram{'A': 0, 'C': 0, 'G': 0, 'T': 0}, nil
	}
	h := Histogram{
		'A': 0,
		'C': 0,
		'G': 0,
		'T': 0,
	}
	nucleotides := map[rune]bool{
		'A': true,
		'C': true,
		'G': true,
		'T': true,
	}
	for i := 0; i < len(d); i++ {
		_, ok := nucleotides[rune(d[i])]
		if !ok {
			return nil, errors.New("invalid nucleotides present")
		}
		h[rune(d[i])]++
	}
	return h, nil
}
