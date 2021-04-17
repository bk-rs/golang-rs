package main

const N = 1

func main() {
	// https://golang.org/ref/spec#ArrayType
	var _ [32]byte
	var _ [2 * N]struct{ x, y int32 }
	var _ [1000]*float64
	var _ [3][5]int
	var _ [2][2][2]float64
}
