package main

import (
	"fmt"
	"strconv"

	"github.com/justinbather/advent-of-code/2025/internal"
)

func main() {

	scanner, err := internal.NewScanner("test.txt")
	if err != nil {
		panic(err)
	}

	defer scanner.Close()

	bank := 0

	for scanner.Scan() {
		line := scanner.Text()
		max := 0
		secondMax := 0

		for _, char := range line {
			charInt, _ := strconv.Atoi(string(char))
			if charInt > max {
				secondMax = max
				max = charInt
				continue
			} else if charInt == max {
				secondMax = charInt
			} else if charInt > secondMax {
				secondMax = charInt
			}
		}

		fmt.Printf("bank %d highest is %d and second is %d\n", bank, max, secondMax)
		bank++
	}

}
