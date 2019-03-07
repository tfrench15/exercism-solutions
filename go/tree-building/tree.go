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

	tree := new(Tree)
	idx, err := findRoot(records)
	if err != nil {
		return nil, err
	}
	records = append(records[:idx], records[idx:]...)
	// TODO: finish...
	return tree.root, nil
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
		return -1, fmt.Errorf("no root node found", nil)
	}
	if count > 1 {
		return -1, fmt.Errorf("duplicate root nodes found", nil)
	}
	return index, nil
}

func isRootNode(record Record) (bool, error) {
	if record.ID == 0 {
		if record.Parent == 0 {
			return true, nil
		}
		return false, fmt.Errorf("root node has parent", nil)
	}
	return false, nil
}

func mergeNodes(parent *Node, records []Record) *Node {
	count := len(records)
	for _, record := range records {
		if record.Parent == parent.ID {
			node := buildNodeFromRecord(record)
			parent.Children = append(parent.Children, node.Children...)
			count--
		}
	}

	if count == 0 {
		return parent
	}
	return parent
}

func buildNodeFromRecord(record Record) *Node {
	return &Node{
		ID:       record.Parent,
		Children: []*Node{&Node{ID: record.ID}},
	}
}
