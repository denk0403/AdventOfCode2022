use std::collections::HashSet;
use std::fs;

const FILE_PATH: &str = "../input.txt";

fn main() {
    let file_content = match fs::read_to_string(FILE_PATH) {
        Ok(str) => str,
        Err(err) => panic!("{err}"),
    };

    let lines = file_content.lines().filter(|line| !line.is_empty());

    let mut priority_sum = 0;
    for line in lines {
        let line_len = line.len();
        let half_length = line_len / 2;
        let first_half = &line[..half_length];
        let second_half = &line[half_length..line_len];

        let seen_chars: HashSet<char> = HashSet::from_iter(first_half.chars());
        for char in second_half.chars() {
            if seen_chars.contains(&char) {
                if char.is_uppercase() {
                    priority_sum += ((char as u8) - ('A' as u8) + 27) as i32;
                } else {
                    priority_sum += ((char as u8) - ('a' as u8) + 1) as i32;
                }
                break;
            }
        }
    }

    println!("{priority_sum}")
}
