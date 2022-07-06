// Problem 1 : Submarine navigation
// Without all the fluff and festive fun of advent of code here is th deets.

// Your submarine starts at position 0,0 (x position, depth)
// Parse input to direct your submarine.
// Multiply your depth * x progression to get an answer

// Example input

// forward 5
// down 5
// forward 8
// up 3
// down 8
// forward 2

// Note
// this means that up 3 is actually -3 from the y axis.

// static string reference: live forever
fn get_input() -> &'static str {
    return "forward 5
down 5
forward 8
up 3
down 8
forward 2";
}

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

fn parse_line(line: &str) -> Point {
    let (dir, amount) = line.split_once(" ").expect("must contain a whitespace");
    // need to transform amount from string to integer
    let amount = str::parse::<i32>(amount).expect("second arg must be an integer");

    if dir == "forward" {
        return Point { x: amount, y: 0 };
    } else if dir == "up" {
        return Point { x: 0, y: -amount };
    }
    return Point { x: 0, y: amount };
}

fn main() {
    let result =
        get_input()
            .lines()
            .map(parse_line)
            .fold(Point { x: 0, y: 0 }, |mut acc, point| {
                acc.x += point.x;
                acc.y += point.y;
                return acc;
            });

    println!("{:?}", result);
    //Point { x: 15, y: 10 }
}
