package main

import (
	"bufio"
	"fmt"
	"log"
	"os"
	"strconv"
	"strings"
)

var keys = []string{"one", "two", "three", "four", "five", "six", "seven", "eight", "nine"}
var nums = []string{"1", "2", "3", "4", "5", "6", "7", "8", "9"}
var numMap = map[string]rune{
	"one":   '1',
	"two":   '2',
	"three": '3',
	"four":  '4',
	"five":  '5',
	"six":   '6',
	"seven": '7',
	"eight": '8',
	"nine":  '9',
}

type numItem struct {
	Idx   int
	Value string
}

func findFirst(line string) numItem {
	var first numItem
	for idx, num := range line {
		if num >= '0' && num <= '9' {

			first.Value = string(num)
			first.Idx = idx
			return first
		}
	}

	return first
}

func findFirstWord(line string) numItem {

	var first numItem
	first.Idx = -1
	for _, num := range keys {
		numIdx := strings.Index(line, num)
		if numIdx != -1 {
			if numIdx <= first.Idx || first.Idx == -1 {
				first.Value = string(numMap[num])
				first.Idx = numIdx
			}
		}
	}

	return first
}

func findLast(line string) numItem {
	var last numItem

	for idx, num := range line {
		if (num >= '0' && num <= '9') && idx >= last.Idx {

			last.Idx = idx
			last.Value = string(num)
		}
	}
	return last
}

func findLastWord(line string) numItem {

	var last numItem

	for _, num := range keys {
		numIdx := strings.LastIndex(line, num)
		fmt.Printf("index: %d, num: %s\n", numIdx, num)
		if numIdx != -1 {
			if numIdx >= last.Idx {
				last.Value = string(numMap[num])
				last.Idx = numIdx
			}
		}
	}

	return last
}

func addVals(s1 numItem, s2 numItem) int {
	fmt.Printf("concating %s and %s\n", s1.Value, s2.Value)
	temp := s1.Value + s2.Value

	sum, err := strconv.Atoi(temp)
	if err != nil {
		log.Fatal(err)
	}

	return sum
}

func main() {

	file, err := os.Open("./day1.txt")
	if err != nil {
		log.Fatal(err)
	}

	defer file.Close()

	scanner := bufio.NewScanner(file)

	var sum int

	for scanner.Scan() {
		line := scanner.Text()
		// find first
		firstNum := findFirst(line)
		firstWord := findFirstWord(line)
		lastNum := findLast(line)
		lastWord := findLastWord(line)

		fmt.Println("last num - last word")
		fmt.Println(lastNum, lastWord)

		var last numItem
		var first numItem

		if (firstNum.Idx < firstWord.Idx) || firstWord.Idx == -1 {
			first = firstNum
		} else {
			first = firstWord
		}

		if (lastNum.Idx > lastWord.Idx) || lastWord.Value == "" {
			last = lastNum
		} else {
			last = lastWord
		}
		fmt.Print(first, last)

		temp := addVals(first, last)
		sum += temp

	}
	if scanner.Err(); err != nil {
		log.Fatal(err)
	}

	fmt.Println("sum: ", sum)
	fmt.Println(sum == 33)
}
