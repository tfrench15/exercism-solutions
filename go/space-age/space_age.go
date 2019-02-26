package space

// Planet represents a planet.
type Planet string

// EarthOrbit is the length of a year on Earth, in seconds.
const EarthOrbit = 31557600

var conversions = map[Planet]float64{
	"Earth":   1,
	"Mercury": 0.2408467,
	"Venus":   0.61519726,
	"Mars":    1.8808158,
	"Jupiter": 11.862615,
	"Saturn":  29.447498,
	"Uranus":  84.016846,
	"Neptune": 164.79132,
}

// Age calculates the age of a person who has lived for the
// given number of seconds, on the given Planet.
func Age(seconds float64, planet Planet) float64 {
	div, ok := conversions[planet]
	if !ok {
		panic("Not a planet")
	}
	earthYrs := seconds / EarthOrbit
	return earthYrs / div
}
