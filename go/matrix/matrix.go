package matrix

import (
	"fmt"
	"strconv"
	"strings"
)

// Matrix represents a matrix.
type Matrix struct {
	rows [][]int
}

// New returns a new Matrix.
func New(s string) (*Matrix, error) {
	m := &Matrix{}
	lines := strings.Split(s, "\n") // [1 2 3, 4 5 6]
	fmt.Println(lines)
	for _, slc := range lines {
		line := strings.Split(slc, " ")
		var nums []int
		for _, elem := range line {
			num, err := strconv.Atoi(elem)
			if err != nil {
				return nil, err
			}
			nums = append(nums, num)
		}
		m.rows = append(m.rows, nums)
	}

	if len(m.rows) > 1 {
		cur := len(m.rows[0])
		for i := 1; i < len(m.rows); i++ {
			if len(m.rows[i]) != cur {
				return nil, fmt.Errorf("error: invalid dimensions; rows of length %d and %d", cur, len(m.rows[i]))
			}
		}
	}

	return m, nil
}

// Rows returns the rows of the Matrix.
func (m *Matrix) Rows() [][]int {
	var rows [][]int

	for _, row := range m.rows {
		var r []int
		for _, entry := range row {
			r = append(r, entry)
		}
		rows = append(rows, r)
	}

	return rows
}

// Cols returns the columns of the Matrix.
func (m *Matrix) Cols() [][]int {
	var columns [][]int
	for i := 0; i < len(m.rows[0]); i++ {
		var column []int
		for j := 0; j < len(m.rows); j++ {
			column = append(column, m.rows[j][i])
		}
		columns = append(columns, column)
	}
	return columns
}

// Set does something I'm not sure about yet.
func (m *Matrix) Set(rows, cols, vals int) bool {
	return false
}
