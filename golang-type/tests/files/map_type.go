package main

type T int

func main() {
	// https://golang.org/ref/spec#MapType
	var _ map[string]int
	var _ map[*T]struct{ x, y float64 }
	var _ map[string]interface{}

	//
	var _ map[string][]string
}
