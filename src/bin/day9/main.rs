use std::{collections::HashSet, fs};

type Coords = (isize, isize);

fn main() {
    let input = fs::read_to_string("src/bin/day9/input.txt").expect("my brain fried");

    println!("Part 1: {}", part_1(&input));
    println!("Part 2: {}", part_2(&input));
}

fn part_1(input: &String) -> usize {
    let starting_position: Coords = (0, 0);

    let mut seen = HashSet::new();

    let mut head = starting_position.clone();
    let mut tail = starting_position.clone();

    seen.insert(tail);

    for line in input.lines() {
        let (dir, amount) = line.split_once(' ').unwrap();
        let amount = amount.parse().unwrap();

        let coords_dir: (isize, isize) = match dir {
            "U" => (0, 1),
            "D" => (0, -1),
            "L" => (1, 0),
            "R" => (-1, 0),
            _ => unreachable!("WHAT ARE YOU DOING YOU LITTLE RASCAL"),
        };

        for _ in 0..amount {
            head = (head.0 + coords_dir.0, head.1 + coords_dir.1);

            let head_to_tail_diff = ((head.0 - tail.0), (head.1 - tail.1));
            let not_touching = head_to_tail_diff.0.abs() > 1 || head_to_tail_diff.1.abs() > 1;

            if not_touching {
                tail.0 += head_to_tail_diff.0.signum();
                tail.1 += head_to_tail_diff.1.signum();
                seen.insert(tail);
            }
        }
    }

    return seen.len();
}

fn part_2(input: &String) -> usize {
    let start: (isize, isize) = (0, 0);

    let mut rope = vec![start; 10];
    let mut seen = HashSet::new();

    seen.insert(start);

    for line in input.lines() {
        let (dir, amount) = line.split_once(' ').unwrap();
        let amount: isize = amount.parse().unwrap();

        for _ in 0..amount {
            match dir {
                "U" => rope[0].0 += 1,
                "D" => rope[0].0 -= 1,
                "L" => rope[0].1 += 1,
                "R" => rope[0].1 -= 1,
                _ => unreachable!("ALKSHDLSA"),
            }

            for (head_ix, tail_ix) in tuple_windows(rope.len()) {
                let diff = (
                    rope[head_ix].0 - rope[tail_ix].0,
                    rope[head_ix].1 - rope[tail_ix].1,
                );

                let not_touching = diff.0.abs() > 1 || diff.1.abs() > 1;

                if not_touching {
                    rope[tail_ix].0 += diff.0.signum();
                    rope[tail_ix].1 += diff.1.signum();
                    if tail_ix == rope.len() - 1 {
                        seen.insert(rope[rope.len() - 1]);
                    }
                }
            }
        }
    }

    return seen.len();
}

fn tuple_windows(len: usize) -> impl Iterator<Item = (usize, usize)> {
    return (0..len - 1).map(|x| (x, x + 1)).into_iter();
}
