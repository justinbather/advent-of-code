package day2

import (
	"bufio"
	"fmt"
	"os"
	"strings"
)

const ROCK int = 1     // A, X
const PAPER int = 2    // B, Y
const SCISSORS int = 3 // C, Z

const WIN int = 6
const TIE int = 3
const LOSE int = 0

func mapInput(input string) int {
	mp := map[string]int{
		"A": ROCK,
		"X": LOSE,

		"B": PAPER,
		"Y": TIE,

		"C": SCISSORS,
		"Z": WIN,
	}

	return mp[input]
}

func Run(test bool) {
	inputFile := "input.txt"
	if test {
		inputFile = "input-test.txt"
	}

	path := "inputs/day2/" + inputFile

	f := open(path)

	s := bufio.NewScanner(f)
	solve(s)
}

/*
A = X = Rock
B = Y = Paper
C = Z = Scissors

*/

func solve(s *bufio.Scanner) {
	sum := 0
	for s.Scan() {
		sum += simulateRound(s.Text())
	}

	fmt.Println("Tournament Score: ", sum)
}

// takes in raw round <P1><space><P2>
// P1 can be oneof: A, B, C
// P2 can be oneof: X, Y, Z
// returns P2 score

func calculateRound(p1, result int) int {
	switch p1 {
	case ROCK:
		if result == WIN {
			return PAPER
		}
		if result == LOSE {
			return SCISSORS
		}

		if result == TIE {
			return ROCK
		}

	case PAPER:
		if result == WIN {
			return SCISSORS
		}
		if result == LOSE {
			return ROCK
		}

		if result == TIE {
			return PAPER
		}
	case SCISSORS:
		if result == WIN {
			return ROCK
		}
		if result == LOSE {
			return PAPER
		}

		if result == TIE {
			return SCISSORS
		}
	}

	panic("shouldnt be here!")
}

func simulateRound(in string) int {
	split := strings.Split(in, " ")

	p1 := mapInput(split[0])
	result := mapInput(split[1])

	score := result + calculateRound(p1, result)

	return score
}

func open(path string) *os.File {
	f, err := os.Open(path)

	if err != nil {
		panic(err)
	}

	return f
}
