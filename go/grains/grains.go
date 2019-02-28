package grains

import (
	"errors"
	"math"
)

// Square returns the number of grains of wheat
// on a particular square of the chessboard.
func Square(num int) (uint64, error) {
	if num <= 0 {
		return 0, errors.New("argument must be an integer >= zero")
	}
	if num > 64 {
		return 0, errors.New("argument must be <= 64")
	}
	fnum := float64(num)
	ans := math.Exp2(fnum - 1)
	return uint64(ans), nil
}

// Total returns the total number of grains of wheat
// on the chessboard.
func Total() uint64 {
	var sum uint64
	for i := 1; i <= 64; i++ {
		sq, err := Square(i)
		if err != nil {
			panic(err)
		}
		sum += sq
	}
	return sum
}
