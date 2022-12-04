mod range;

use range::Range;
use std::{fs, str::FromStr};

fn main() {
    let input = fs::read_to_string("./src/bin/day4/input.txt").expect("where input");
    let input = input.trim();

    let mut fully_contained = 0;
    let mut parially_contained = 0;

    for line in input.split("\n") {
        let (left, right) = line.split_once(",").unwrap(); // guaranteed to have A-B,X-Z format

        let left = Range::from_str(left).unwrap();
        let right = Range::from_str(right).unwrap();

        if Range::either_fully_contains_another(&left, &right) {
            fully_contained += 1;
        }

        if Range::either_partially_contains_another(&left, &right) {
            parially_contained += 1;
        }
    }

    println!("Part 1: {}", fully_contained);
    println!("Part 2: {}", parially_contained);
}
