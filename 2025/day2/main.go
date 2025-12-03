package main

import (
	"fmt"
	"strconv"
	"strings"

	"github.com/justinbather/advent-of-code/2025/day2/i"
	"github.com/justinbather/advent-of-code/2025/internal"
)

func main() {

	scanner, err := internal.NewScanner("input.txt")
	if err != nil {
		panic(err)
	}

	defer scanner.Close()

	total := 0

	for scanner.Scan() {
		line := scanner.Text()
		parts := parseLine(line)

		//     11-22
		for _, part := range parts {
			// 11  22
			start, end := parsePart(part)
			ids := i.CompileIds(start, end)

			for _, id := range ids {
				if !i.ValidId(strconv.Itoa(id)) {
					fmt.Printf("invalid id %d\n", id)
					total += id
				}
			}

		}
	}

	fmt.Printf("total: %d\n", total)
}

func parseLine(line string) []string {
	return strings.Split(line, ",")
}

func parsePart(part string) (int, int) {
	idRange := strings.Split(part, "-")
	start, err := strconv.Atoi(idRange[0])
	if err != nil {
		panic(err)
	}

	end, err := strconv.Atoi(idRange[1])
	if err != nil {
		panic(err)
	}

	return start, end
}
