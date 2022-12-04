use std::fs;

const FILE_PATH: &str = "../input.txt";

fn main() {
    let file_content = match fs::read_to_string(FILE_PATH) {
        Ok(str) => str,
        Err(err) => panic!("{err}"),
    };

    let range_pairs = file_content.lines().filter_map(|line| {
        if line.is_empty() {
            return None;
        } else {
            Some(
                line.split(",")
                    .map(|range| {
                        range
                            .split("-")
                            .map(|num_str| num_str.parse::<u32>().unwrap())
                            .collect::<Vec<_>>()
                    })
                    .collect::<Vec<_>>(),
            )
        }
    });

    let mut count = 0;
    for pair in range_pairs {
        let range1 = &pair[0][0]..=&pair[0][1];
        let range2 = &pair[1][0]..=&pair[1][1];

        if range1.contains(&range2.start())
            || range1.contains(&range2.end())
            || range2.contains(&range1.start())
            || range2.contains(&range1.end())
        {
            count += 1;
        }
    }

    println!("{:?}", count)
}
