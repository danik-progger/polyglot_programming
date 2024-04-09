function generateInput(): string {
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

enum Cell { 
  Tree,
  Snow
} 

const cells = generateInput().split("\n")
  .map(x =>
    x.split("")
      .map(x => x === "." ? Cell.Snow : Cell.Tree)
  )

const colLen = cells[0].length; 
let treesCount = 0;

cells.forEach((row, i) => {
  if (row[i * 3 % colLen] === Cell.Tree) {
    ++treesCount;
  }
});
 