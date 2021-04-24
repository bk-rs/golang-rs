package main

type Node = int
type Comparable = int

type (
	nodeList = []*Node // nodeList and []*Node are identical types
	Polar    = polar   // Polar and polar denote identical types
)

type (
	Point struct{ x, y float64 } // Point and struct{ x, y float64 } are different types
	polar Point                  // polar and Point denote different types
)

type TreeNode struct {
	left, right *TreeNode
	value       *Comparable
}

type Foo struct {
	bar uint
}

func main() {
}
