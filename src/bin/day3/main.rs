mod rucksack;

use rucksack::Rucksack;
use std::{fs, str::FromStr};

fn generate_priorities() -> Vec<char> {
    return (b'a'..=b'z').chain(b'A'..=b'Z').map(char::from).collect();
}

fn get_priority(c: &char, priorities: &Vec<char>) -> usize {
    return priorities.iter().position(|item| c == item).unwrap() + 1;
}

fn main() {
    let input = fs::read_to_string("./src/bin/day3/input.txt").expect("where input");
    let input = input.trim();

    let priorities = generate_priorities();

    // part 1
    let mut priorities_sum: usize = 0;

    for raw_rucksack in input.split("\n") {
        let rucksack = Rucksack::from_str(raw_rucksack).unwrap();
        let items_in_both_compartments = rucksack.in_both_compartments();
        priorities_sum += items_in_both_compartments
            .iter()
            .fold(0, |acc, letter| acc + get_priority(letter, &priorities));
    }

    println!("Part 1: {}", priorities_sum);

    // part 2
    let input: Vec<&str> = input.split("\n").collect();
    let mut priorities_sum: usize = 0;

    for elve_group in input.chunks(3) {
        let group_rucksacks = elve_group
            .iter()
            .map(|line| Rucksack::from_str(line).unwrap())
            .collect::<Vec<Rucksack>>();

        let common_items = Rucksack::common_items(&group_rucksacks);
        for item in common_items {
            priorities_sum += get_priority(item, &priorities);
        }
    }

    println!("Part 2: {}", priorities_sum);
}
