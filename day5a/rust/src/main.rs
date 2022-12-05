use std::fs;

const FILE_PATH: &str = "../input.txt";

fn main() {
    let Ok(file_content) = fs::read_to_string(FILE_PATH) else {
        panic!("Can't read file {FILE_PATH}");
    };

    let [stack_part, instructions_part] =
        file_content.split("\n\n").collect::<Vec<&str>>()[..] else {
            panic!("Can't parse parts of the file.");
        };

    let stack_lines = parse_stack_part(stack_part);
    let mut stacks = create_stack_from_lines(stack_lines);

    let instructions = parse_instructions(instructions_part);

    perform_instructions(&mut stacks, instructions);

    for stack in stacks {
        print!("{}", stack.last().unwrap());
    }
    println!();
}

fn parse_stack_part(stack_part: &str) -> Vec<Vec<char>> {
    stack_part
        .lines()
        .map(|line| line.chars().skip(1).step_by(4).collect())
        .collect()
}

fn create_stack_from_lines(stack_lines: Vec<Vec<char>>) -> Vec<Vec<char>> {
    let total_stacks = stack_lines[0].len();
    let stacks_height = stack_lines.len() - 1;

    let mut stacks = vec![vec![]; total_stacks];

    for row in (0..stacks_height).rev() {
        let line = &stack_lines[row];
        for (index, char) in line.iter().enumerate() {
            if !char.is_whitespace() {
                stacks[index].push(*char);
            }
        }
    }

    return stacks;
}

fn parse_instructions(instructions_part: &str) -> Vec<Vec<usize>> {
    instructions_part
        .lines()
        .map(|line| {
            line.split(" ")
                .skip(1)
                .step_by(2)
                .map(|num| num.parse().unwrap())
                .collect()
        })
        .collect()
}

fn perform_instructions(
    stacks: &mut Vec<Vec<char>>,
    instructions: Vec<Vec<usize>>,
) -> &mut Vec<Vec<char>> {
    for instruction in instructions {
        let remove_amount = instruction[0];
        let from = instruction[1] - 1;
        let to = instruction[2] - 1;

        let mut removed = vec![];
        for _ in 0..remove_amount {
            let popped = stacks[from].pop().unwrap();
            removed.push(popped);
        }

        stacks[to].append(&mut removed);
    }

    return stacks;
}
