use std::{fs, slice::Iter};

const FILE_PATH: &str = "../input.txt";

const TOTAL_SPACE: usize = 70_000_000;
const NEEDED_SPACE: usize = 30_000_000;

struct Dir(Vec<Dir>, usize);

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

    let mut command_iter = command_groups.iter();

    let root = build_dir(&mut command_iter);
    let space_used = root.1;
    let free_space = TOTAL_SPACE - space_used;
    let real_space_needed = NEEDED_SPACE - free_space;

    let size = find_min_space(&root, real_space_needed).unwrap();

    println!("{:?}", size);
}

fn build_dir(command_iter: &mut Iter<Vec<Vec<&str>>>) -> Dir {
    let (mut dirs, mut dir_size) = (vec![], 0);
    while let Some(command_group) = command_iter.next() {
        let command = &command_group[0];

        match command[0] {
            "cd" => match command[1] {
                ".." => {
                    break;
                }
                _ => dirs.push(build_dir(command_iter)),
            },
            "ls" => {
                let output = &command_group[1..];
                for line in output {
                    if let Ok(size) = line[0].parse::<usize>() {
                        dir_size += size
                    }
                }
            }
            &_ => {}
        }
    }

    dir_size += dirs.iter().map(|d| d.1).sum::<usize>();
    Dir(dirs, dir_size)
}

fn find_min_space(dir: &Dir, min_size: usize) -> Option<usize> {
    dir.0
        .iter()
        .filter(|d| d.1 >= min_size)
        .flat_map(|d| [Some(d.1), find_min_space(d, min_size)])
        .flatten()
        .min()
}
