package main

import (
	"fmt"
	"os"
	"sort"
	"strconv"
	"strings"
)

const FILE_PATH = "../input.txt"

func main() {
	input, err := os.ReadFile(FILE_PATH)

	if err == nil {
		lines := strings.Split(string(input), "\n")
		running_total := 0
		totals := []int{}

		for _, line := range lines {
			if line == "" {
				totals = append(totals, running_total)
				running_total = 0
				continue
			}

			number, err := strconv.ParseInt(line, 10, 0)
			if err == nil {
				running_total += int(number)
			}
		}

		sort.Ints(totals)

		// Reverse array into descending
		for i, j := 0, len(totals)-1; i < j; i, j = i+1, j-1 {
			totals[i], totals[j] = totals[j], totals[i]
		}

		fmt.Println(totals[0] + totals[1] + totals[2])
	}

}
