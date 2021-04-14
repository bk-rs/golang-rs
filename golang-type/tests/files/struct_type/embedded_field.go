package main

import (
	P "time"
)

type T1 int
type T2 int

func main() {
	// https://golang.org/ref/spec#StructType
	var _ struct {
		T1             // field name is T1
		*T2            // field name is T2
		P.Duration     // field name is Duration
		*P.Month       // field name is Month
		x, y       int // field names are x and y
	}
}
