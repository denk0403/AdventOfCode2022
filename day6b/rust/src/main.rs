use std::collections::HashSet;

const NON_REPEAT_COUNT: usize = 14;

fn main() {
    let windows = include_bytes!("../../input.txt").windows(NON_REPEAT_COUNT);

    let mut result = 0;
    for (index, chars) in windows.enumerate() {
        let set: HashSet<&u8> = HashSet::from_iter(chars.iter());

        if set.len() == NON_REPEAT_COUNT {
            result = index + NON_REPEAT_COUNT;
            break;
        }
    }

    println!("{result}");
}
