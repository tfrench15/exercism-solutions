package matrix

// Matrix represents a matrix.
type Matrix struct {
	rows [][]string
}

// New returns a new Matrix.
func New(s string) (*Matrix, error) {
	return &Matrix{}, nil
}

// Rows returns the rows of the Matrix.
func (m *Matrix) Rows() [][]string {
	return m.rows
}

// Cols returns the columns of the Matrix.
func (m *Matrix) Cols() [][]string {
	// do some transforms
	return m.rows
}
