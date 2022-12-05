use std::fs;

fn main() {
    let input = fs::read_to_string("./src/bin/day1/input.txt").unwrap();
    let input = input.trim();

    let mut calories: Vec<u32> = input
        .split("\n\n")
        .map(|chunk| {
            chunk
                .split("\n")
                .map(|num| num.parse::<u32>().unwrap())
                .sum()
        })
        .collect();

    calories.sort_by(|a, b| b.cmp(a));

    let part_one = calories[0];
    let part_two: u32 = calories.iter().take(3).sum();

    println!("Part one: {}", part_one);
    println!("Part two: {}", part_two);
}
