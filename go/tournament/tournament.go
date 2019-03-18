package tournament

import (
	"fmt"
	"io"
)

// Tally tallies and reports the results of a tournament.
func Tally(reader io.Reader, writer io.Writer) error {
	b := make([]byte, 0)
	reader.Read(b)
	strs := string(b)
	fmt.Println(strs)
	return nil
}
