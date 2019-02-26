package collatzconjecture

import "fmt"

// CollatzConjecture takes a starting number and returns the
// number of steps required to reach 1 by continually applying
// the Collatz Conjecture.
func CollatzConjecture(num int) (int, error) {
	if num < 1 {
		return -1, fmt.Errorf("given number %d is <= 0", num)
	}
	steps := 0
	for {
		if num == 1 {
			return steps, nil
		}
		if num%2 == 0 {
			num = num / 2
			steps++
			continue
		}
		num = 3*num + 1
		steps++
		continue
	}
}
