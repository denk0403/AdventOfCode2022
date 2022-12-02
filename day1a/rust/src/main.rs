use std::fs;

const FILE_PATH: &str = "../input.txt";

fn main() {
    let file_content = match fs::read_to_string(FILE_PATH) {
        Ok(str) => str,
        Err(err) => panic!("{err}"),
    };

    let (_, max_calories) = file_content.lines().fold(
        (0, 0),
        |(running_calorie_total, max_calories_so_far), line| {
            if line.is_empty() {
                if running_calorie_total > max_calories_so_far {
                    (0, running_calorie_total)
                } else {
                    (0, max_calories_so_far)
                }
            } else {
                let calorie_amount: u32 = line.parse().unwrap();
                (running_calorie_total + calorie_amount, max_calories_so_far)
            }
        },
    );

    println!("{max_calories}");
}
