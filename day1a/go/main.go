package main

import (
	"fmt"
	"os"
	"strconv"
	"strings"
)

const FILE_PATH = "../input.txt"

func main() {
	input, err := os.ReadFile(FILE_PATH)

	if err == nil {
		lines := strings.Split(string(input), "\n")
		running_total := 0
		max_so_far := 0

		for _, line := range lines {
			if line == "" {
				if running_total > max_so_far {
					max_so_far = running_total
				}
				running_total = 0
				continue
			}

			number, err := strconv.ParseInt(line, 10, 0)
			if err == nil {
				running_total += int(number)
			}
		}

		fmt.Println(max_so_far)
	}

}
