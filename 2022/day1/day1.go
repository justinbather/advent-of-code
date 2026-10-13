package day1

import (
	"bufio"
	"fmt"
	"os"
	"slices"
	"strconv"
)

type elf struct {
	Index int
	Cals  int
}

func newElf(index int, cals int) elf {
	return elf{index, cals}
}

func Run(test bool) {
	inputFile := "input.txt"
	if test {
		inputFile = "input-test.txt"
	}

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

	f, err := os.Open(inputFile)

	if err != nil {
		panic(err)
	}

	defer f.Close()

	s := bufio.NewScanner(f)

	elves := []elf{}

	currElfCals := 0

	for s.Scan() {
		line := s.Text()
		if len(line) == 0 {
			elves = append(elves, newElf(len(elves), currElfCals))
			currElfCals = 0
			continue
		}

		cals, _ := strconv.Atoi(line)
		currElfCals += cals
	}

	// last elf
	elves = append(elves, newElf(len(elves), currElfCals))

	slices.SortFunc(elves, func(a, b elf) int {
		return a.Cals - b.Cals
	})

	fmt.Println(elves)

	sum := 0
	for _, elf := range elves[len(elves)-3:] {
		sum += elf.Cals
	}

	fmt.Printf("sum is %d", sum)
}
