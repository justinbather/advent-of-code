package main

import (
	"fmt"
	"strconv"
	"strings"

	"github.com/justinbather/advent-of-code/2025/internal"
)

// 27** too low
// 5956 too high
// 5934 too high

const (
	LEFT  = "L"
	RIGHT = "R"
)

func main() {
	scanner, err := internal.NewScanner("test-input.txt")
	if err != nil {
		panic(err)
	}

	defer scanner.Close()

	counter := counter{state: 50}

	for scanner.Scan() {
		line := scanner.Text()
		if len(line) > 0 {

			cmd, err := parseLine(line)
			if err != nil {
				panic(err)
			}

			counter.update(cmd)
		}
	}

	if err := scanner.Err(); err != nil {
		panic(err)
	}

	fmt.Printf("FINAL %d\n", counter.state)
	fmt.Println(counter.passedZero)
}

func parseLine(line string) (command, error) {
	dir := string(line[0])
	numStr := strings.TrimLeft(line, "LR")
	dist, err := strconv.Atoi(numStr)

	return command{dir: dir, dist: dist}, err
}

type command struct {
	dir  string
	dist int
}

type counter struct {
	state      int
	passedZero int
}

func (c *counter) update(cmd command) {
	switch cmd.dir {
	case LEFT:
		for i := range cmd.dist {
			if i == 0 && c.state == 0 {
				c.state = 99
				continue
			}

			if c.state == 0 {
				c.state = 99
				c.passedZero++
			} else {
				c.state--
			}
		}

	default:
		for i := range cmd.dist {
			if i == 0 && c.state == 0 {
				c.state++
				continue
			}
			if c.state == 99 {
				c.state = 0
				c.passedZero++
			} else {
				c.state++
			}
		}
	}
}
