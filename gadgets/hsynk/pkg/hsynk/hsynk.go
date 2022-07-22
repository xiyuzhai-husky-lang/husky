package main

import (
	"flag"
	"fmt"
)

func main() {
	num := flag.Int("n", 5, "# of iterations")
	flag.Parse()

	n := *num
	fmt.Println("n = ", n)
}
