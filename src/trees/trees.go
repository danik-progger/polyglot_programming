package main

import (
	"fmt"
	"strings"
)


func generateInput() string {
  return `
    ..##.......
    #...#...#..
    .#....#..#.
    ..#.#...#.#
    .#...##..#. 
    ..#.##.....
    .#.#.#....#
    .#........#
    #.##...#...
    #...##....#
    .#..#...#.#
    `;
} 


type Cell = int
const (
	Tree Cell = iota
	Snow
)

func main() {
	treesCount := 0
	for row, line := range strings.Split(generateInput(), "\n") {
		if (line[row * 3 %  len(line)]) == '#' {
			treesCount += 1
		}
	}

	fmt.Printf("%v", treesCount)
}