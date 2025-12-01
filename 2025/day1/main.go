package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
	"strings"
)

func main() {

	f, err := os.Open("input.txt")
	if err != nil {
		panic(err)
	}
	defer f.Close()

	scanner := bufio.NewScanner(f)

	curr := 50
	numZeros := 0

	for scanner.Scan() {
		line := scanner.Text()
		if len(line) > 0 {
			dir := line[0]
			numStr := strings.TrimLeft(line, "LR")
			dist, err := strconv.Atoi(numStr)
			if err != nil {
				panic(err)
			}
			if dir == 'L' { //Left
				curr = curr - dist
				for {
					if curr >= 0 {
						break
					}

					if curr < 0 {
						curr = curr + 100
					}
				}
			} else { // Right
				curr = curr + dist
				for {
					if curr <= 99 {
						break
					}

					if curr > 99 {
						curr = curr - 100
					}
				}
			}

			if curr == 0 {
				fmt.Printf("%s to point at 0\n", line)
				numZeros++
			}
		}
	}

	if err := scanner.Err(); err != nil {
		panic(err)
	}

	fmt.Println(numZeros)

}
