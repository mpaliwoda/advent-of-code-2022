use std::fs;

#[derive(Clone, Copy)]
struct Range {
    start: u32,
    finish: u32,
}

fn main() {
    let input = fs::read_to_string("./src/bin/day4/input.txt").expect("where input");
    let input = input.trim();
    let mut fully_contained = 0;
    let mut parially_contained = 0;

    for line in input.split("\n") {
        let (left, right) = line.split_once(",").unwrap(); // guaranteed to have A-B,X-Z format
        let (left_range, right_range) = (range(left), range(right));

        let (lower, upper) = if left_range.start < right_range.start
            || left_range.start == right_range.start && left_range.finish >= right_range.finish
        {
            (left_range, right_range)
        } else {
            (right_range, left_range)
        };

        if upper.start >= lower.start && upper.finish <= lower.finish {
            fully_contained += 1;
        }

        if upper.start <= lower.finish {
            parially_contained += 1;
        }
    }

    println!("Part 1: {}", fully_contained);
    println!("Part 2: {}", parially_contained);
}

fn range(from: &str) -> Range {
    let (start, finish) = from.split_once("-").unwrap();
    return Range {
        start: start.parse().expect("?"),
        finish: finish.parse().expect("?"),
    };
}
