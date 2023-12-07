package main

import (
	"bufio"
	"fmt"
	"log"
	"os"
	"strings"
)

func parseGame(line string) int {

	var score int

	game := strings.Split(line, ":")
	allNums := strings.Split(game[1], "|")

	winningNums := strings.Split(allNums[0], " ")

	ticketNums := strings.Split(allNums[1], " ")
	fmt.Println(len(ticketNums))

	for _, winner := range winningNums {

		for _, ticketNum := range ticketNums {
			if winner == ticketNum && winner != " " && winner != "" {
				fmt.Println(winner, ticketNum)
				if score == 0 {
					score = 1
				} else {

					fmt.Println(score)
					score = score * 2
					fmt.Println(score)
				}
			}
		}
	}

	fmt.Println(allNums, score)

	return score
}

func main() {

	file, err := os.Open("./day4.txt")
	if err != nil {
		log.Fatal(err)
	}
	defer file.Close()

	scanner := bufio.NewScanner(file)

	var score int
	for scanner.Scan() {
		line := scanner.Text()
		score += parseGame(line)
	}

	fmt.Println(score)

	if scanner.Err(); err != nil {
		log.Fatal(err)
	}

}
