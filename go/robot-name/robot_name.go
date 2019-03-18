package robotname

import (
	"math/rand"
	"strconv"
	"time"
)

const letters = "ABCDEFGHIJKLMNOPQRSTUVWXYZ"

var seed = rand.New(rand.NewSource(time.Now().UnixNano()))

// Robot represents a robot coming off our assembly line.  It
// must have a unique name of the form 'LLDDD' where 'L' is an
// upper-case letter and 'D' is a digit.
type Robot struct {
	name string
}

// Name generates a name for the Robot if one has not already
// been set.
func (r *Robot) Name() (string, error) {
	if r.name != "" {
		return r.name, nil
	}

	chars := generateLetters(letters)
	digs := generateDigits()

	r.name = chars + digs

	return r.name, nil
}

// Reset resets the robot's name to the empty string.
func (r *Robot) Reset() {
	r.name = ""
}

func generateLetters(charset string) string {
	b := make([]byte, 2)
	for i := range b {
		b[i] = charset[seed.Intn(len(letters))]
	}
	return string(b)
}

func generateDigits() string {
	d1 := seed.Intn(10)
	d2 := seed.Intn(10)
	d3 := seed.Intn(10)

	s1 := strconv.Itoa(d1)
	s2 := strconv.Itoa(d2)
	s3 := strconv.Itoa(d3)

	return s1 + s2 + s3
}
