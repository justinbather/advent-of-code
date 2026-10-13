package day3

import (
	"bufio"
	"fmt"
	"os"
)

var priority = func() map[rune]int {
	m := make(map[rune]int)
	for i, r := range "abcdefghijklmnopqrstuvwxyz" {
		m[r] = i + 1
	}
	for i, r := range "ABCDEFGHIJKLMNOPQRSTUVWXYZ" {
		m[r] = i + 27
	}
	return m
}()

func Run(test bool) {
	inputFile := "input.txt"
	if test {
		inputFile = "input-test.txt"
	}

	path := "inputs/day3/" + inputFile

	f := open(path)

	s := bufio.NewScanner(f)

	curr := 0
	buffer := []string{}

	sum := 0
	for s.Scan() {
		curr++

		if curr <= 3 {
			line := s.Text()
			buffer = append(buffer, line)
		}

		if curr == 3 {
			badge, err := findBadge(buffer)
			if err != nil {
				panic(err)
			}
			sum += priority[badge]
			curr = 0
			buffer = []string{}
			continue
		}
	}

	fmt.Printf("sum of priority %d\n", sum)

}

func findBadge(rucks []string) (rune, error) {
	seen := make(map[rune]struct{})
	for _, r := range rucks[0] {
		seen[r] = struct{}{}
	}

	inBoth := make(map[rune]struct{})
	for _, r := range rucks[1] {
		if _, ok := seen[r]; ok {
			inBoth[r] = struct{}{}
		}
	}

	for _, r := range rucks[2] {
		if _, ok := inBoth[r]; ok {
			return r, nil
		}
	}

	return 0, fmt.Errorf("badge not found in %v", rucks)
}

func open(path string) *os.File {
	f, err := os.Open(path)

	if err != nil {
		panic(err)
	}

	return f
}
