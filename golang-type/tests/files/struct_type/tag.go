package main

func main() {
	// https://golang.org/ref/spec#StructType
	var _ struct {
		microsec  uint64 `protobuf:"1"`
		serverIP6 uint64 `protobuf:"2"`
	}
}
