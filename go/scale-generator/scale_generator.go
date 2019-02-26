package scale

import "strings"

var (
	sharpScale  = []string{"A", "A#", "B", "C", "C#", "D", "D#", "E", "F", "F#", "G", "G#"}
	flatScale   = []string{"A", "Bb", "B", "C", "Db", "D", "Eb", "E", "F", "Gb", "G", "Ab"}
	intervalMap = map[string]int{"m": 1, "M": 2, "A": 3}
	useSharps   = map[string]bool{
		"C":  true,
		"G":  true,
		"D":  true,
		"A":  true,
		"a":  true,
		"E":  true,
		"B":  true,
		"F#": true,
		"e":  true,
		"b":  true,
		"f#": true,
		"c#": true,
		"g#": true,
		"d#": true,
	}
)

// Scale generates the musical scale based on the
// given interval (steps between notes) and tonic
// (starting note).
func Scale(tonic, interval string) []string {
	scale := []string{}
	if interval == "" {
		if useSharps[tonic] {
			idx := getTonicIndex(strings.Title(tonic), sharpScale)
			for i := idx; i < idx+len(sharpScale); i++ {
				scale = append(scale, sharpScale[i%len(sharpScale)])
			}
			return scale
		}
		idx := getTonicIndex(strings.Title(tonic), flatScale)
		for i := idx; i < idx+len(flatScale); i++ {
			scale = append(scale, flatScale[i%len(flatScale)])
		}
		return scale
	}

	if useSharps[tonic] {
		idx := getTonicIndex(strings.Title(tonic), sharpScale)
		for i := 0; i < len(interval); i++ {
			scale = append(scale, sharpScale[idx%12])
			idx += intervalMap[string(interval[i])]
		}
		return scale
	}
	idx := getTonicIndex(strings.Title(tonic), flatScale)
	for i := 0; i < len(interval); i++ {
		scale = append(scale, flatScale[idx%12])
		idx += intervalMap[string(interval[i])]
	}
	return scale
}

func getTonicIndex(tonic string, slc []string) int {
	for i, elem := range slc {
		if elem == tonic {
			return i
		}
	}
	return -1
}
