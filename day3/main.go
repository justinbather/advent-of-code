package main

import (
	"bufio"
	"fmt"
	"log"
	"os"
	"strconv"
)

// Parse through text
// Find all numbers, saving the start and end index, line number
// Find all special chars, saving the index, and line number
// Loop over the numbers map

type specChar struct {
	Literal string
	Index   int
	Line    int
}

type part struct {
	Value    int
	StartIdx int
	EndIdx   int
	Line     int
	Valid    bool
}

var partsFound = []part{}
var specChars = []specChar{}

// Parse through text
// Find all numbers, saving the start and end index, line number
// Find all special chars, saving the index, and line number
// Loop over the numbers map

func parseLine(line string, lineNum int) {

	for i := 0; i < (len(line)); i++ {

		if line[i] >= '0' && line[i] <= '9' {

			var tempVal string
			var j int
			for j = i; j < (len(line)) && (line[j] >= '0' && line[j] <= '9'); j++ {
				tempVal += string(line[j])
			}
			val, err := strconv.Atoi(tempVal)

			if err != nil {
				log.Fatal(err, lineNum, j)
			}

			tempPart := part{Value: val, StartIdx: i, EndIdx: i + (len(tempVal) - 1), Line: lineNum, Valid: false}
			partsFound = append(partsFound, tempPart)

			i += len(tempVal) - 1
		} else {

			if line[i] == '*' {
				tmp := string(line[i])
				tempChar := specChar{Literal: tmp, Index: i, Line: lineNum}
				specChars = append(specChars, tempChar)

			}
		}

		// if char is a number
		// while char is number
		// add byte to number
		// else if char is spec char
		// create part

	}
}

func main() {

	file, err := os.Open("./day3.txt")
	if err != nil {
		log.Fatal(err)
	}

	defer file.Close()

	scanner := bufio.NewScanner(file)

	var lineNum int
	for scanner.Scan() {

		lineNum++
		line := scanner.Text()
		parseLine(line, lineNum)

	}

	var sum int

	type adj struct {
		numAdj int
		vals   []int
		symbol specChar
	}

	type adjPair struct {
		part    part
		char    specChar
		matched bool
	}

	adjPairs := []adjPair{}
	for _, part := range partsFound {
		for _, specChar := range specChars {
			fmt.Println("comparing ", part, specChar)
			if (part.Line == (specChar.Line+1) || part.Line == (specChar.Line-1) || part.Line == specChar.Line) &&
				((specChar.Index+1) >= part.StartIdx && (specChar.Index-1) <= part.EndIdx) {
				fmt.Println(part)

				adjPairs = append(adjPairs, adjPair{part: part, char: specChar, matched: false})

			}
		}
	}

	for j, outerPair := range adjPairs {

		for _, innerPair := range adjPairs[j+1:] {
			if (innerPair.char == outerPair.char) && (innerPair.matched == false) && (outerPair.matched == false) {
				sum += (innerPair.part.Value * outerPair.part.Value)
				fmt.Println("Matched: ", outerPair.part.Value, innerPair.part.Value)
				innerPair.matched = true
			}
		}
	}

	fmt.Println(specChars)

	fmt.Println(sum)
	if err := scanner.Err(); err != nil {
		log.Fatal(err)
	}

}
