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
		"X": ROCK,

		"B": PAPER,
		"Y": PAPER,

		"C": SCISSORS,
		"Z": SCISSORS,
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

func calculateRoundWon(p1, p2 int) int {
	switch p2 {
	case ROCK:
		if p1 == ROCK {
			return TIE
		}
		if p1 == PAPER {
			return LOSE
		}

		if p1 == SCISSORS {
			return WIN
		}

	case PAPER:

		if p1 == ROCK {
			return WIN
		}
		if p1 == PAPER {
			return TIE
		}

		if p1 == SCISSORS {
			return LOSE
		}
	case SCISSORS:

		if p1 == ROCK {
			return LOSE
		}
		if p1 == PAPER {
			return WIN
		}

		if p1 == SCISSORS {
			return TIE
		}
	}

	panic("shouldnt be here!")
}

func simulateRound(in string) int {
	split := strings.Split(in, " ")

	p1 := mapInput(split[0])
	p2 := mapInput(split[1])

	score := p2 + calculateRoundWon(p1, p2)

	return score
}

func open(path string) *os.File {
	f, err := os.Open(path)

	if err != nil {
		panic(err)
	}

	return f
}
