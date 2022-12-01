use std::fs;

const FILE_PATH: &str = "input.txt";

fn main() {
    let file_content = match fs::read_to_string(FILE_PATH) {
        Ok(str) => str,
        Err(err) => panic!("{err}"),
    };

    let (_, mut all_calorie_totals) = file_content.lines().fold(
        (0, vec![]),
        |(current_running_calorie_total, mut collected_calorie_totals), line| {
            if line.is_empty() {
                collected_calorie_totals.push(current_running_calorie_total);
                (0, collected_calorie_totals)
            } else {
                let calorie_amount: u32 = line.parse().unwrap();
                (
                    current_running_calorie_total + calorie_amount,
                    collected_calorie_totals,
                )
            }
        },
    );

    all_calorie_totals.sort_by(|c1, c2| c2.cmp(c1));

    let sum_top_3 = all_calorie_totals[0] + all_calorie_totals[1] + all_calorie_totals[2];
    println!("{sum_top_3}");
}
