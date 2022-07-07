// Problem 3 : Trees and something like free ski
// AdventOfCode - Day 3

// You are given the following input:

// ..##.......
// #...#...#..
// .#....#..#.
// ..#.#...#.#
// .#...##..#.
// ..#.##.....
// .#.#.#....#
// .#........#
// #.##...#...
// #...##....#
// .#..#...#.#
// It wraps over and over again

// ..##.........##.......
// #...#...#..#...#...#..
// .#....#..#..#....#..#.
// ..#.#...#.#..#.#...#.#
// .#...##..#..#...##..#.
// ..#.##.......#.##..... ---- > repeats for ever!
// .#.#.#....#.#.#.#....#
// .#........#.#........#
// #.##...#...#.##...#...
// #...##....##...##....#
// .#..#...#.#.#..#...#.#
// We need to travel 3 right and 1 down and count how many trees we hit.

// # = tree
// . = snow

// Example input

// ..##.......
// #...#...#..
// .#....#..#.
// ..#.#...#.#
// .#...##..#.
// ..#.##.....
// .#.#.#....#
// .#........#
// #.##...#...
// #...##....#
// .#..#...#.#

// Note
// You don't get the size.

fn get_input() -> &'static str {
    return "..##.......
#...#...#..
.#....#..#.
..#.#...#.#
.#...##..#.
..#.##.....
.#.#.#....#
.#........#
#.##...#...
#...##....#
.#..#...#.#";
}

fn main() {
    let result = get_input()
        .lines()
        .enumerate()
        .flat_map(|(idx, line)| return line.chars().nth(idx * 3 % line.len()))
        .filter(|&x| x == '#')
        .count();

    println!("{}", result)
    // 7
}
