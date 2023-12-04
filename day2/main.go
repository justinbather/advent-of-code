package main

import (
	"bufio"
	"fmt"
	"log"
	"os"
	"strconv"
	"strings"
)

var max = map[string]int{"red": 12, "blue": 14, "green": 13}

func getGameId(line string) int {

	strArr := strings.Split(line, ":")
	strId := strings.Split(strArr[0], " ")

	id, err := strconv.Atoi(strId[1])
	if err != nil {
		log.Fatal(err)
	}

	return id

}

func parseGames(line string) int {

	//remove game id
	gameArr := strings.Split(line, ":")
	//Seperates rounds
	roundArr := strings.Split(gameArr[1], ";")

	for _, rnd := range roundArr {
		clr := strings.Split(rnd, ",")

		for _, i := range clr {
			curr := strings.Split(i, " ")
			numDice, err := strconv.Atoi(curr[1])
			if err != nil {
				log.Fatal(err)
			}

			if numDice > max[curr[2]] {
				return 0
			}
		}
	}

	gameId := getGameId(line)
	return gameId
}

func main() {
	/*
	 *
	 * 12 red, 13 green, 14 blue
	 *
	 */

	file, err := os.Open("./day2.txt")
	if err != nil {
		log.Fatal(err)
	}
	defer file.Close()

	scanner := bufio.NewScanner(file)

	var sum int

	for scanner.Scan() {
		line := scanner.Text()
		id := parseGames(line)
		sum += id
	}

	if scanner.Err(); err != nil {
		log.Fatal(err)
	}

	fmt.Println(sum)
}
