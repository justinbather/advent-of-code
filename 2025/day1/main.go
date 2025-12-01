package main

import (
	"fmt"
	"strconv"
	"strings"

	"github.com/justinbather/advent-of-code/2025/internal"
)

const (
	LEFT  = "L"
	RIGHT = "R"
)

func main() {
	var (
		curr     = 50
		numZeros = 0
	)
	scanner, err := internal.NewScanner("input.txt")
	if err != nil {
		panic(err)
	}

	defer scanner.Close()

	for scanner.Scan() {
		line := scanner.Text()
		if len(line) > 0 {

			dir, dist, err := parseLine(line)
			if err != nil {
				panic(err)
			}

			if dir == LEFT {
				curr = curr - dist
				for {
					if curr >= 0 {
						break
					}

					if curr < 0 {
						curr = curr + 100
					}
				}
			} else {
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
				numZeros++
			}
		}
	}

	if err := scanner.Err(); err != nil {
		panic(err)
	}

	fmt.Println(numZeros)

}

func parseLine(line string) (dir string, dist int, err error) {
	dir = string(line[0])
	numStr := strings.TrimLeft(line, "LR")
	dist, err = strconv.Atoi(numStr)

	return dir, dist, err
}
