use std::fs;

const FILE_PATH: &str = "../input.txt";

fn main() {
    let Ok(file_content) = fs::read_to_string(FILE_PATH) else {
        panic!("Can't read file {FILE_PATH}");
    };

    let windows = file_content.as_bytes().windows(4);

    let mut result = 0;
    for (index, chars) in windows.enumerate() {
        let [ch1, ch2, ch3, ch4] = chars else {
            panic!("Error parsing window of 4 characters");
        };

        if ch1 != ch2 && ch1 != ch3 && ch1 != ch4 && ch2 != ch3 && ch2 != ch4 && ch3 != ch4 {
            result = index + 4;
            break;
        }
    }

    println!("{result}");
}
