package main

import (
	"bufio"
	"fmt"
	"os"
	"slices"
	"strconv"
)

type Elf struct {
	Index int
	Cals  int
}

func NewElf(index int, cals int) Elf {
	return Elf{index, cals}
}

func main() {
	day1()
}

func day1() {
	/*
		input:
		1000
		2000
		3000

		4000

		5000
		6000

		7000
		8000
		9000

		10000
	*/

	f, err := os.Open("input.txt")
	if err != nil {
		panic(err)
	}

	defer f.Close()

	s := bufio.NewScanner(f)

	elves := []Elf{}

	currElfCals := 0

	for s.Scan() {
		line := s.Text()
		if len(line) == 0 {
			elves = append(elves, NewElf(len(elves), currElfCals))
			currElfCals = 0
			continue
		}

		cals, _ := strconv.Atoi(line)
		currElfCals += cals
	}

	// last elf
	elves = append(elves, NewElf(len(elves), currElfCals))

	slices.SortFunc(elves, func(a, b Elf) int {
		return a.Cals - b.Cals
	})

	fmt.Println(elves)

	sum := 0
	for _, elf := range elves[len(elves)-3:] {
		sum += elf.Cals
	}

	fmt.Printf("sum is %d", sum)

}
