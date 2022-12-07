use std::{fs, slice::Iter};

const FILE_PATH: &str = "../input.txt";

fn main() {
    let Ok(file_content) = fs::read_to_string(FILE_PATH) else {
        panic!("Can't read file {FILE_PATH}");
    };

    let command_groups: Vec<Vec<Vec<&str>>> = file_content
        .split("$ ")
        .skip(1)
        .map(|command_group| {
            command_group
                .lines()
                .map(|line| line.split(" ").collect())
                .collect()
        })
        .collect();

    let mut size_of_dirs_under_100k = 0;
    let mut command_iter = command_groups.iter();

    calc_size_of_under_100k(&mut command_iter, &mut size_of_dirs_under_100k);

    println!("{:?}", size_of_dirs_under_100k);
}

fn calc_size_of_under_100k(command_iter: &mut Iter<Vec<Vec<&str>>>, sum: &mut usize) -> usize {
    let mut dir_size = 0;
    while let Some(command_group) = command_iter.next() {
        let command = &command_group[0];

        match command[0] {
            "cd" => match command[1] {
                ".." => {
                    break;
                }
                _ => dir_size += calc_size_of_under_100k(command_iter, sum),
            },
            "ls" => {
                let output = &command_group[1..];
                for line in output {
                    match line[0] {
                        "dir" => {}
                        arg1 => {
                            if let Ok(size) = arg1.parse::<usize>() {
                                dir_size += size
                            }
                        }
                    };
                }
            }
            &_ => {}
        }
    }

    if dir_size < 100_000 {
        *sum += dir_size
    }

    dir_size
}
