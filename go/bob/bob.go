package bob

import (
	"strings"
	"unicode"
)

// Hey returns hard-coded responses to certain sentence types.
func Hey(remark string) string {
	remark = strings.TrimSpace(remark)
	if isShouting(remark) && isQuestion(remark) {
		return "Calm down, I know what I'm doing!"
	}

	if isQuestion(remark) {
		return "Sure."
	}

	if isShouting(remark) {
		return "Whoa, chill out!"
	}

	if isSilent(remark) {
		return "Fine. Be that way!"
	}

	return "Whatever."
}

func hasLetters(remark string) bool {
	for i := 0; i < len(remark); i++ {
		if unicode.IsLetter(rune(remark[i])) {
			return true
		}
	}
	return false
}

func isQuestion(remark string) bool {
	if strings.LastIndex(remark, "?") == -1 {
		return false
	}
	if strings.LastIndex(remark, "?") == len(remark)-1 {
		return true
	}
	return false
}

func isShouting(remark string) bool {
	if hasLetters(remark) && strings.ToUpper(remark) == remark {
		return true
	}
	return false
}

func isSilent(remark string) bool {
	if strings.TrimSpace(remark) == "" || remark == "" {
		return true
	}
	return false
}
