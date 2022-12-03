use std::collections::HashSet;
use std::fs;

const FILE_PATH: &str = "../input.txt";

fn main() {
    let file_content = match fs::read_to_string(FILE_PATH) {
        Ok(str) => str,
        Err(err) => panic!("{}", err),
    };

    let mut lines = file_content
        .lines()
        .filter(|line| !line.is_empty())
        .peekable();

    let mut priority_sum = 0;

    loop {
        let first_bag = lines.next().unwrap();
        let second_bag = lines.next().unwrap();
        let third_bag = lines.next().unwrap();

        let mut common_chars_set: HashSet<char> = HashSet::from_iter(first_bag.chars());
        let second_bag_char_set: HashSet<char> = HashSet::from_iter(second_bag.chars());
        common_chars_set.retain(|item| second_bag_char_set.contains(item));

        for char in third_bag.chars() {
            if common_chars_set.contains(&char) {
                if char.is_uppercase() {
                    priority_sum += ((char as u8) - ('A' as u8) + 27) as i32;
                } else {
                    priority_sum += ((char as u8) - ('a' as u8) + 1) as i32;
                }
                break;
            }
        }

        if lines.peek() == None {
            break;
        }
    }

    println!("{priority_sum}")
}
