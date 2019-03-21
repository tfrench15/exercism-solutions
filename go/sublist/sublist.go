package sublist

// Relation is a string that indicates the relationship
// between two lists.
type Relation string

// Sublist returns the relationship between two given lists.
// It can be any of 'equal', 'sublist','superlist', or 'unequal'.
func Sublist(l1, l2 []int) Relation {
	len1, len2 := len(l1), len(l2)

	if len1 == len2 {
		if ok := isEqual(l1, l2); ok {
			return "equal"
		}
		return "unequal"
	}

	if len1 < len2 {
		i, j := 0, len1
		for j < len2 {
			ok := isEqual(l1, l2[i:j])
			if ok {
				return "sublist"
			}
			i++
			j++
		}
		ok := isEqual(l1, l2[i:])
		if ok {
			return "sublist"
		}
	}

	if len1 > len2 {
		i, j := 0, len2
		for j < len1 {
			ok := isEqual(l1[i:j], l2)
			if ok {
				return "superlist"
			}
			i++
			j++
		}
		ok := isEqual(l1[i:], l2)
		if ok {
			return "superlist"
		}
	}
	return "unequal"
}

func isEqual(l1, l2 []int) bool {
	for i := range l1 {
		if l1[i] != l2[i] {
			return false
		}
	}
	return true
}
