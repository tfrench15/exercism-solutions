package letter

// FreqMap records the frequency of each rune in a given text.
type FreqMap map[rune]int

// Frequency counts the frequency of each rune in a given text and returns this
// data as a FreqMap.
func Frequency(s string) FreqMap {
	m := make(FreqMap)
	for _, r := range s {
		m[r]++
	}
	return m
}

// ConcurrentFrequency calls Frequency() concurrently.
func ConcurrentFrequency(slc []string) FreqMap {
	freqMap := make(FreqMap)
	ch := make(chan FreqMap, len(slc))
	go func(strs []string) {
		for _, str := range strs {
			fm := Frequency(str)
			ch <- fm
		}
		close(ch)
	}(slc)

	for m := range ch {
		for k, v := range m {
			freqMap[k] += v
		}
	}
	return freqMap
}
