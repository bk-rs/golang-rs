package main

func main() {
	// https://golang.org/ref/spec#StructType
	var _ struct {
		x, y int
		u    float32
		_    float32 // padding
		A    *[]int
		F    func()
	}
}
