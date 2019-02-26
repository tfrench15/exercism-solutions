package strand

// ToRNA transcribes a given strand of DNA into
// a strand of RNA.
func ToRNA(dna string) string {
	if dna == "" {
		return ""
	}
	rna := ""
	m := map[string]string{
		"G": "C",
		"C": "G",
		"T": "A",
		"A": "U",
	}
	for i := 0; i < len(dna); i++ {
		comp := m[string(dna[i])]
		rna += comp
	}
	return rna
}
