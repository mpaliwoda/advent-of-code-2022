use std::{collections::HashSet, fs};

fn main() {
    let input = fs::read_to_string("./src/bin/day3/input.txt").expect("where input");
    let input = input.trim();

    let mut prio: Vec<char> = (b'a'..=b'z').map(char::from).collect();
    prio.extend((b'A'..=b'Z').map(char::from));

    println!("{}", part1(input, &prio));
    println!("{}", part2(input, &prio));
}

fn part1(input: &str, priorities: &Vec<char>) -> usize {
    let mut prio_sum: usize = 0;

    for rucksack in input.split("\n") {
        let mid = rucksack.len() / 2; // rucksack len is guaranteed to be divisible by two
        let (left, right) = rucksack.split_at(mid);
        let (left, right): (HashSet<&str>, HashSet<&str>) = (
            HashSet::from_iter(left.split("").filter(|&s| s != "")),
            HashSet::from_iter(right.split("").filter(|&s| s != "")),
        );

        let in_both = left.intersection(&right);

        for item in in_both {
            let item = item.chars().next().unwrap();
            prio_sum += priorities
                .iter()
                .position(|&letter| letter == item)
                .unwrap()
                + 1;
        }
    }

    return prio_sum;
}

fn part2(input: &str, priorities: &Vec<char>) -> usize {
    let input: Vec<&str> = input.split("\n").collect();
    let mut prio_sum: usize = 0;

    for elve_group in input.chunks(3) {
        let mut group_items = elve_group
            .iter()
            .map(|elve| HashSet::<char>::from_iter(elve.chars()));

        let acc = group_items.next().unwrap();
        let in_all = group_items.fold(acc, |acc, set| acc.intersection(&set).cloned().collect());

        assert!(in_all.len() == 1);

        let in_all = in_all.iter().next().unwrap();
        prio_sum += priorities
            .iter()
            .position(|&letter| &letter == in_all)
            .unwrap()
            + 1;
    }

    return prio_sum;
}
