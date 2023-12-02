package main

import (
	"bufio"
	"fmt"
	"log"
	"os"
	"strconv"
)

func main() {

	file, err := os.Open("./day1.txt")
	if err != nil {
		log.Fatal(err)
	}

	defer file.Close()

	scanner := bufio.NewScanner(file)

	var sum int

	for scanner.Scan() {

		var first string
		var last string
		line := scanner.Text()
		for _, char := range line {

			if string(char) >= "0" && string(char) <= "9" {
				if first == "" {
					//Grab first correct value found
					first = string(char)
				}
				last = string(char)
				// Greedy search for last valid value
				fmt.Println(last)
			}
		}
		temp := first + last
		tempSum, err := strconv.Atoi(temp)
		if err != nil {
			log.Fatal(err)
		}
		sum += tempSum

	}
	fmt.Println(sum)

	if scanner.Err(); err != nil {
		log.Fatal(err)
	}

}
