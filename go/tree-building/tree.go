package tree

type Record struct {
	ID     int
	Parent int
}

// Node represents an element in a tree.
type Node struct {
	ID       int
	Children []*Node
}

// Build builds a tree of Nodes given a slice of records.
func Build(records []Record) (*Node, error) {
	if len(records) == 0 {
		return nil, nil
	}
	if len(records) == 1 {
		return &Node{
			ID: records[0].ID,
		}, nil
	}
	return nil, nil
}

// FindParent takes a slice of records and returns the parent of
// the tree.
func FindParent(records []Record) (*Node, error) {
	parent := records[0]
	for _, record := range records {
		if record.ID < parent.ID {
			parent = record
		}
	}
	return nil, nil
}
