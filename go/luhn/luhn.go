package luhn

import (
	"fmt"
	"strconv"
	"unicode"
)

// Valid determines whether a given string is valid
// per the Luhn forumla.
func Valid(s string) bool {
	// check length
	s = removeSpaces(s)
	fmt.Println(s)
	if len(s) <= 1 {
		return false
	}

	if ok := hasNonDigits(s); ok {
		return false
	}

	double := map[string]string{
		"0": "0",
		"1": "2",
		"2": "4",
		"3": "6",
		"4": "8",
		"5": "1",
		"6": "3",
		"7": "5",
		"8": "7",
		"9": "9",
	}

	var revLuhn string
	skip := true
	for i := len(s) - 1; i >= 0; i-- {
		if skip {
			revLuhn += string(s[i])
			skip = false
			continue
		}
		dig := double[string(s[i])]
		revLuhn += dig
		skip = true
	}

	luhn := reverseString(revLuhn)

	sum := 0
	for i := 0; i < len(luhn); i++ {
		num, err := strconv.Atoi(string(luhn[i]))
		if err != nil {
			return false
		}
		sum += num
	}
	if sum%10 == 0 {
		return true
	}
	return false
}

func removeSpaces(s string) string {
	var ret string
	for _, r := range s {
		if unicode.IsSpace(r) {
			continue
		}
		ret += string(r)
	}
	return ret
}

func hasNonDigits(s string) bool {
	for _, r := range s {
		if !unicode.IsDigit(r) {
			return true
		}
	}
	return false
}

func reverseString(s string) string {
	var ret string
	for i := len(s) - 1; i >= 0; i-- {
		ret += string(s[i])
	}
	return ret
}
