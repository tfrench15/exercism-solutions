package proverb

import "fmt"

// Proverb outputs a proverb that depends upon the given
// slice of strings.
func Proverb(rhyme []string) []string {
	if len(rhyme) == 0 {
		return []string{}
	}
	proverb := []string{}
	last := fmt.Sprintf("And all for the want of a %s.", rhyme[0])
	for len(rhyme) > 1 {
		cur := rhyme[0]
		next := rhyme[1]
		line := fmt.Sprintf("For want of a %s the %s was lost.", cur, next)
		proverb = append(proverb, line)
		rhyme = rhyme[1:]
	}
	proverb = append(proverb, last)
	return proverb
}
