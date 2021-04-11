package main

import (
	"encoding/json"
	"fmt"
)

type S struct {
	A int `json:""`
	B int `json:"-"`
	C int `json:"-,"`
	D int `json:"d"`
	E int `json:",omitempty"`
	F int `json:",string"`
	G int `json:"g,omitempty"`
	H int `json:"h,string"`
	I int `json:"i,omitempty,string"`
	J int `json:"j,string,omitempty"`
	K int `json:"k,foo,bar"`
	L int `json:"l" xml:""`
}

func main() {
	s1 := &S{
		A: 1,
		B: 1,
		C: 1,
		D: 1,
		E: 1,
		F: 1,
		G: 1,
		H: 1,
		I: 1,
		J: 1,
		K: 1,
		L: 1,
	}
	str1, _ := json.Marshal(s1)
	fmt.Println(string(str1))

	s0 := &S{
		A: 0,
		B: 0,
		C: 0,
		D: 0,
		E: 0,
		F: 0,
		G: 0,
		H: 0,
		I: 0,
		J: 0,
		K: 0,
		L: 0,
	}
	str0, _ := json.Marshal(s0)
	fmt.Println(string(str0))
}
