package triangle

import "math"

// Kind represents a kind of Triangle.
type Kind string

const (
	NaT Kind = "NaT"
	Equ Kind = "Equ"
	Iso Kind = "Iso"
	Sca Kind = "Sca"
)

// KindFromSides takes three side lengths and returns
// the kind of Triangle they make.
func KindFromSides(a, b, c float64) Kind {
	isTriangle := IsTriangle(a, b, c)
	if !isTriangle {
		return NaT
	}
	if a == b && b == c {
		return Equ
	}
	if a == b || a == c || b == c {
		return Iso
	}
	return Sca
}

// IsTriangle checks whether the given side lengths
// pass the Triangle Inequality, and other basic constraints.
func IsTriangle(a, b, c float64) bool {
	if math.IsNaN(a) || math.IsNaN(b) || math.IsNaN(c) {
		return false
	}
	if math.IsInf(a, 1) || math.IsInf(b, 1) || math.IsInf(c, 1) {
		return false
	}
	if a <= 0 || b <= 0 || c <= 0 {
		return false
	}
	if a+b < c || a+c < b || b+c < a {
		return false
	}
	return true
}
