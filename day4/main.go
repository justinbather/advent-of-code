package main

import (
	"bufio"
	"fmt"
	"log"
	"os"
	"strings"
)

var tickets map[int]int = make(map[int]int)

func parseGame(line string) int {

	var score int

	game := strings.Split(line, ":")
	allNums := strings.Split(game[1], "|")

	winningNums := strings.Split(allNums[0], " ")

	ticketNums := strings.Split(allNums[1], " ")

	for _, winner := range winningNums {

		for _, ticketNum := range ticketNums {
			if winner == ticketNum && winner != " " && winner != "" {
				score++
			}
		}
	}

	return score
}

func main() {

	var ticketNumber int
	file, err := os.Open("./day4.txt")
	if err != nil {
		log.Fatal(err)
	}
	defer file.Close()

	scanner := bufio.NewScanner(file)

	for scanner.Scan() {
		ticketNumber++
		line := scanner.Text()
		winningNums := parseGame(line)
		tickets[ticketNumber] += 1
		// give each ticket after current a copy for each winning num in current
		for i := 1; i <= winningNums; i++ {
			tickets[ticketNumber+i] += 1 * tickets[ticketNumber]
		}

	}

	//Calculate sum
	var sum int
	for _, value := range tickets {
		sum += value
	}
	fmt.Println(sum)

	if scanner.Err(); err != nil {
		log.Fatal(err)
	}

}
