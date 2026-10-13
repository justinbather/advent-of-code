package main

import (
	"fmt"
	"os"

	"github.com/justinbather/aoc/2022/day1"
	"github.com/justinbather/aoc/2022/day2"
	"github.com/spf13/cobra"
)

type day struct {
	name string
	run  func(test bool)
}

var days = []day{
	{name: "day1", run: day1.Run},
	{name: "day2", run: day2.Run},
}

func main() {
	root := &cobra.Command{
		Use:   "aoc",
		Short: "Advent of Code 2022",
	}

	for _, d := range days {
		var test bool
		cmd := &cobra.Command{
			Use:   d.name,
			Short: fmt.Sprintf("Run %s", d.name),
			Run: func(cmd *cobra.Command, args []string) {
				d.run(test)
			},
		}
		cmd.Flags().BoolVar(&test, "test", false, "use input-test.txt")
		root.AddCommand(cmd)
	}

	if err := root.Execute(); err != nil {
		fmt.Println(err)
		os.Exit(1)
	}
}
