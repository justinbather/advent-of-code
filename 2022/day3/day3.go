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

	common := []rune{}

	for s.Scan() {
		line := s.Text()

		freqFirst := make(map[rune]int)
		firstHalf := line[:len(line)/2]
		for _, char := range firstHalf {
			_, ok := freqFirst[char]
			if !ok {
				freqFirst[char] = 1
			} else {
				freqFirst[char]++
			}
		}

		freqSec := make(map[rune]int)
		secondHalf := line[len(line)/2:]
		for _, char := range secondHalf {
			_, ok := freqSec[char]
			if !ok {
				freqSec[char] = 1
			} else {
				freqSec[char]++
			}
		}

		for k := range freqFirst {
			_, ok := freqSec[k]
			if !ok {
				continue
			} else {
				common = append(common, k)
			}
		}
	}

	sum := 0
	for _, val := range common {
		sum += priority[val]
	}

	fmt.Printf("total priority is %d\n", sum)
}

func open(path string) *os.File {
	f, err := os.Open(path)

	if err != nil {
		panic(err)
	}

	return f
}
