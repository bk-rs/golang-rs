package main

import (
	"time"
)

type foo int

func main() {
	//
	var _ bool

	//
	var _ uint8
	var _ uint16
	var _ uint32
	var _ uint64

	var _ int8
	var _ int16
	var _ int32
	var _ int64

	var _ float32
	var _ float64

	var _ complex64
	var _ complex128

	var _ byte
	var _ rune

	var _ uint
	var _ int
	var _ uintptr

	//
	var _ string

	//
	var _ time.Duration

	//
	var _ foo
}
