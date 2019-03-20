package tree

import "fmt"

// Record represents the ID of a node and the ID of its
// parent.
type Record struct {
	ID     int
	Parent int
}

// Node represents an element in a tree.
type Node struct {
	ID       int
	Children []*Node
}

// ByID represents a slice of nodes satisfying the
// sort.Interface interface.
type ByID []*Node

// Len returns the length of the slice of nodes.
func (n ByID) Len() int {
	return len(n)
}

// Less reports which of the given nodes has the lesser ID.
func (n ByID) Less(i, j int) bool {
	return n[i].ID <= n[j].ID
}

// Swap flips the nodes at the given indices.
func (n ByID) Swap(i, j int) {
	n[i], n[j] = n[j], n[i]
}

// Tree represents a tree of nodes.
type Tree struct {
	root *Node
}

// Build builds a tree of Nodes given a slice of records.
func Build(records []Record) (*Node, error) {
	// trivial cases
	if len(records) == 0 {
		return nil, nil
	}
	if len(records) == 1 {
		return &Node{
			ID:       records[0].ID,
			Children: nil,
		}, nil
	}

	rootIdx, err := findRoot(records)
	if err != nil {
		return nil, err
	}
	rootRec := records[rootIdx]
	root := &Node{ID: rootRec.ID}

	parents := []int{0}
	records = append(records[:rootIdx], records[rootIdx:]...)
	for _, record := range records {
		insertNode(root, record)
	}

}

// findRoot returns the location of the root node in the slice
// of Records.  An error is returned if there is no root node,
// or if there are duplicate root nodes.
func findRoot(records []Record) (int, error) {
	count := 0
	index := -1
	for i, record := range records {
		ok, err := isRootNode(record)
		if err != nil {
			return -1, err
		}
		if ok {
			index = i
		}
	}
	if count < 1 {
		return -1, fmt.Errorf("no root node found")
	}
	if count > 1 {
		return -1, fmt.Errorf("duplicate root nodes found")
	}
	return index, nil
}

// isRootNode determines whether a given record is a valid root node.
func isRootNode(record Record) (bool, error) {
	if record.ID == 0 {
		if record.Parent == 0 {
			return true, nil
		}
		return false, fmt.Errorf("root node has parent with ID: %d", record.Parent)
	}
	return false, nil
}

// insertNode inserts a given record into its parent node's
// array of children.
func insertNode(parent *Node, record Record) {
	if record.Parent == parent.ID {
		child := &Node{ID: record.ID}
		parent.Children = append(parent.Children, child)
	}
}

// mergeNodes checks that the given nodes have the same ID.
// If they do, the nodes' list of children are merged.  Else,
// an error is returned to the caller.
func mergeNodes(n1, n2 *Node) (*Node, error) {
	if n1.ID != n2.ID {
		return nil, fmt.Errorf("cannot merge nodes: node ID %d does not match node ID %d", n1.ID, n2.ID)
	}
	n1.Children = append(n1.Children, n2.Children...)
	return n1, nil
}

// buildNodeFromRecord creates a Node from the given record.
func buildNodeFromRecord(record Record) *Node {
	return &Node{
		ID:       record.Parent,
		Children: []*Node{&Node{ID: record.ID}},
	}
}
