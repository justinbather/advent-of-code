package main

import (
	"bufio"
	"fmt"
	"log"
	"os"
)

func main() {

	fmt.Println("day5")
	file, err := os.Open("./day5.txt")
	if err != nil {
		log.Fatal(err)
	}
	defer file.Close()

	scanner := bufio.NewScanner(file)

	for scanner.Scan() {

	}

	//Calculate sum

	if scanner.Err(); err != nil {
		log.Fatal(err)
	}

}
