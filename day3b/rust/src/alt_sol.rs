use std::fs;

const FILE_PATH: &str = "../input.txt";

fn main() {
    let file_content = match fs::read_to_string(FILE_PATH) {
        Ok(str) => str,
        Err(err) => panic!("{}", err),
    };

    let lines = file_content
        .lines()
        .filter(|line| !line.is_empty())
        .collect::<Vec<_>>();
    let chunks = lines.chunks(3);

    let mut priority_sum = 0;

    for chunk in chunks {
        let common_char = chunk[0]
            .chars()
            .find(|&char| chunk[1].contains(char) && chunk[2].contains(char))
            .unwrap() as u8;

        // is lower case
        if common_char >= b'a' {
            priority_sum += (common_char - b'a') as u16 + 1;
        } else {
            priority_sum += (common_char - b'A') as u16 + 27;
        }
    }

    println!("{priority_sum}")
}
