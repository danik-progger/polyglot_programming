fn generate_input() -> &'static str {
  return "
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
    ";
}
 
enum Cell {
    Tree,
    Snow
}  
 
fn main() {
    let ans = generate_input()
        .lines()
        .enumerate()
        .flat_map(|(idx, line)| {
            return line
                .chars()
                .nth(idx * 3 % line.len());
        })
        .filter(|&x| x =="#")
        .count();

    println!("{:?}", ans);
}