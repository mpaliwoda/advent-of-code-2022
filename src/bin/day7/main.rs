use std::fs;

const MAX_SOUGHT_DIR_SIZE: usize = 100_000;

const TOTAL_AVAILABLE_SPACE: usize = 70_000_000;
const UPDATE_SIZE: usize = 30_000_000;

fn main() {
    let input = fs::read_to_string("./src/bin/day7/input.txt").expect("you make me un poco loco");

    let mut stack: Vec<(&str, usize)> = vec![];
    let mut total = 0;
    let mut all_dirs: Vec<(&str, usize)> = vec![];

    for line in input.lines() {
        let tokens: Vec<&str> = line.split_whitespace().collect();

        assert!(tokens.len() > 1);

        match tokens[..] {
            ["$", "ls"] => continue,
            ["$", "cd", ".."] => {
                let (name, size) = stack.pop().unwrap();
                stack.last_mut().unwrap().1 += size;

                if size < MAX_SOUGHT_DIR_SIZE {
                    total += size;
                }

                all_dirs.push((name, size));
            }
            ["$", "cd", dir] => stack.push((dir, 0)),
            ["dir", _] => continue,
            [raw_size, _] => {
                let size = raw_size.parse::<usize>().unwrap();
                stack.last_mut().unwrap().1 += size;
            }
            _ => unreachable!("AOC error?"),
        }
    }

    while stack.len() > 0 {
        let (name, size) = stack.pop().unwrap();
        all_dirs.push((name, size));

        if stack.len() > 0 {
            stack.last_mut().unwrap().1 += size;
        }
    }

    let current_free_space = TOTAL_AVAILABLE_SPACE - all_dirs.last().unwrap().1;
    let need_to_delete = UPDATE_SIZE - current_free_space;

    let candidate_size = all_dirs
        .iter()
        .filter(|(_, size)| *size >= need_to_delete)
        .map(|(_, size)| size)
        .min()
        .unwrap();

    println!("Part 1: {}", total);
    println!("Part 2: {}", candidate_size);
}
