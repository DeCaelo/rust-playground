// Problem 2 : Fissures
// No festivities

// You are given lines ((x1, y1) -> (x2, y2))
// Only consider horizontal and vertical lines
// We wont solve the actual problem as its a bit verbose!

// Example input

// 0,9 -> 5,9
// 8,0 -> 0,8
// 9,4 -> 3,4
// 2,2 -> 2,1
// 7,0 -> 7,4
// 6,4 -> 2,0
// 0,9 -> 2,9
// 3,4 -> 1,4
// 0,0 -> 8,8
// 5,5 -> 8,2

use anyhow::anyhow;
use std::str::FromStr;

fn get_input() -> &'static str {
    return "0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2";
}

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}
#[derive(Debug)]
struct Line {
    p1: Point,
    p2: Point,
}

// is a trait to interface with fct avaivable
impl FromStr for Point {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> anyhow::Result<Self> {
        let result = s.split_once(",");
        if result.is_none() {
            return Err(anyhow!("expected a point to contain a comma"));
        }

        let (x, y) = result.unwrap();
        let x: i32 = str::parse(x)?;
        let y: i32 = str::parse(y)?;

        return Ok(Point { x, y });
    }
}

impl FromStr for Line {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> anyhow::Result<Self> {
        let result = s.split_once(" -> ");
        if result.is_none() {
            return Err(anyhow!("expected a line to contain ->"));
        }

        let (p1, p2) = result.unwrap();
        let p1: Point = str::parse(p1)?;
        let p2: Point = str::parse(p2)?;

        return Ok(Line { p1, p2 });
    }
}

impl Line {
    // &self: i need an instance but cannot mutate data.
    fn is_horv(&self) -> bool {
        return self.p1.x == self.p2.x || self.p1.y == self.p2.y;
    }
}

fn main() {
    let lines = get_input()
        .lines()
        .flat_map(str::parse)
        .filter(|x: &Line| x.is_horv())
        .collect::<Vec<Line>>();

    println!("{:?}", lines);
    // [
    //     Line {
    //         p1: Point { x: 0, y: 9 },
    //         p2: Point { x: 5, y: 9 },
    //     },
    //     Line {
    //         p1: Point { x: 9, y: 4 },
    //         p2: Point { x: 3, y: 4 },
    //     },
    //     Line {
    //         p1: Point { x: 2, y: 2 },
    //         p2: Point { x: 2, y: 1 },
    //     },
    //     Line {
    //         p1: Point { x: 7, y: 0 },
    //         p2: Point { x: 7, y: 4 },
    //     },
    //     Line {
    //         p1: Point { x: 0, y: 9 },
    //         p2: Point { x: 2, y: 9 },
    //     },
    //     Line {
    //         p1: Point { x: 3, y: 4 },
    //         p2: Point { x: 1, y: 4 },
    //     },
    // ]
}
