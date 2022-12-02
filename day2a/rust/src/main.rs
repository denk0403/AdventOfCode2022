// use std::fmt;
use std::fs;

const FILE_PATH: &str = "../input.txt";

#[derive(PartialEq, Eq, Clone, Copy)]
enum Shape {
    Rock,
    Paper,
    Scissors,
}

// impl fmt::Display for Shape {
//     // This trait requires `fmt` with this exact signature.
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         let name = match self {
//             Shape::Rock => "Rock",
//             Shape::Paper => "Paper",
//             Shape::Scissors => "Scissors",
//         };
//         write!(f, "{}", name)
//     }
// }

fn key2shape(key: char) -> Shape {
    match key {
        'A' => Shape::Rock,
        'B' => Shape::Paper,
        'C' => Shape::Scissors,
        'X' => Shape::Rock,
        'Y' => Shape::Paper,
        'Z' => Shape::Scissors,
        _ => panic!("Unknown key: {key}"),
    }
}

fn shape2score(shape: Shape) -> i32 {
    match shape {
        Shape::Rock => 1,
        Shape::Paper => 2,
        Shape::Scissors => 3,
    }
}

#[derive(PartialEq, Eq, Clone, Copy)]
enum Result {
    Win,
    Lose,
    Draw,
}

fn result2score(result: Result) -> i32 {
    match result {
        Result::Win => 6,
        Result::Lose => 0,
        Result::Draw => 3,
    }
}

fn get_round_result(op_shape: Shape, me_shape: Shape) -> Result {
    if op_shape == me_shape {
        return Result::Draw;
    }

    if (op_shape == Shape::Rock && me_shape == Shape::Scissors)
        || (op_shape == Shape::Paper && me_shape == Shape::Rock)
        || (op_shape == Shape::Scissors && me_shape == Shape::Paper)
    {
        return Result::Lose;
    }

    Result::Win
}

fn main() {
    let file_content = match fs::read_to_string(FILE_PATH) {
        Ok(str) => str,
        Err(err) => panic!("{err}"),
    };

    let lines = file_content.lines().filter(|line| !line.is_empty());

    let mut my_score = 0;
    for line in lines {
        let line_bytes = line.as_bytes();
        let op_key = line_bytes[0] as char;
        let me_key = line_bytes[2] as char;

        let op_shape = key2shape(op_key);
        let me_shape = key2shape(me_key);

        let result = get_round_result(op_shape, me_shape);

        my_score += result2score(result) + shape2score(me_shape)
    }

    println!("{my_score}")
}
