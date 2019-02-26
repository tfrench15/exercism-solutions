package listops

import "reflect"

// IntList is a slice of integers.
type IntList []int

type binFunc func(x, y int) int

// Foldr is a method that 'right-folds' the elements of
// an IntList.
func (il IntList) Foldr(bf binFunc, initial int) int {
	if len(il) == 0 {
		return initial
	}
	i := len(il) - 1
	ret := bf(il[i], initial)
	i--
	for i >= 0 {
		ret = bf(il[i], ret)
		i--
	}
	return ret
}

// Foldl is a method that 'left-folds' the elements of
// an IntList.
func (il IntList) Foldl(bf binFunc, initial int) int {
	if len(il) == 0 {
		return initial
	}
	i := 0
	ret := bf(initial, il[i])
	i++
	for i < len(il) {
		ret = bf(ret, il[i])
		i++
	}
	return ret
}

type predFunc func(n int) bool

// Filter filters il using the given predFunc.
func (il IntList) Filter(pf predFunc) IntList {
	ret := []int{}
	if len(il) == 0 {
		return IntList(ret)
	}
	for _, num := range il {
		if pf(num) {
			ret = append(ret, num)
		}
	}
	return IntList(ret)
}

// Length is a method returning the length of the IntList.
func (il IntList) Length() int {
	length := 0
	if reflect.DeepEqual(il, IntList([]int{})) {
		return length
	}
	for i := range il {
		length = i + 1
	}
	return length
}

type unaryFunc func(x int) int

// Map is a method returning a mapping of the IntList
// to a new IntList after applying the unaryFunc.
func (il IntList) Map(uf unaryFunc) IntList {
	ret := make(IntList, 0)
	if il.Length() == 0 {
		return ret
	}

	for _, num := range il {
		ans := uf(num)
		ret = append(ret, ans)
	}
	return ret
}

// Reverse is a method that reverses the IntList.
func (il IntList) Reverse() IntList {
	ret := make(IntList, 0)
	if il.Length() == 0 {
		return ret
	}
	for i := il.Length() - 1; i >= 0; i-- {
		ret = append(ret, il[i])
	}
	return ret
}

// Append is a method that appends the newIL to il.
func (il IntList) Append(newIL IntList) IntList {
	if reflect.DeepEqual(newIL, IntList([]int{})) {
		return il
	}
	if reflect.DeepEqual(il, IntList([]int{})) {
		return newIL
	}
	ret := make([]int, il.Length()+newIL.Length(), il.Length()+newIL.Length())
	for i, elem := range il {
		ret[i] = elem
	}
	j := 0
	for i := il.Length(); i < il.Length()+newIL.Length(); i++ {
		ret[i] = newIL[j]
		j++
	}
	return IntList(ret)
}

// Concat is a method that concatenates slc with il.
func (il IntList) Concat(slices []IntList) IntList {
	if len(slices) == 0 {
		return il
	}
	for _, slice := range slices {
		il = il.Append(slice)
	}
	return il
}
