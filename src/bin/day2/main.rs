use std::fs;

const ROCK: u32 = 1;
const PAPER: u32 = 2;
const SCISSORS: u32 = 3;

const WIN: u32 = 6;
const DRAW: u32 = 3;
const LOSS: u32 = 0;

fn main() {
    let input = fs::read_to_string("./src/bin/day2/input.txt").expect("y it no here");
    let input = input.trim();

    println!("{}", part1(input));
    println!("{}", part2(input));
}

fn part1(input: &str) -> u32 {
    fn letter_to_hand(letter: &str) -> u32 {
        match letter {
            "A" | "X" => ROCK,
            "B" | "Y" => PAPER,
            "C" | "Z" => SCISSORS,
            _ => panic!("oh shit"),
        }
    }

    let mut score: u32 = 0;

    for line in input.split("\n") {
        let (a, b) = line.split_once(" ").unwrap();
        let (a, b) = (letter_to_hand(a), letter_to_hand(b));

        let mut round_score = b;
        let result = match (b, a) {
            (ROCK, PAPER) => LOSS,
            (ROCK, SCISSORS) => WIN,
            (PAPER, ROCK) => WIN,
            (PAPER, SCISSORS) => LOSS,
            (SCISSORS, ROCK) => LOSS,
            (SCISSORS, PAPER) => WIN,
            (a, b) if a == b => DRAW,
            _ => panic!("oh shit 2"),
        };

        round_score += result;

        score += round_score;
    }

    return score;
}

fn part2(input: &str) -> u32 {
    fn letter_to_hand(letter: &str) -> u32 {
        match letter {
            "A" => ROCK,
            "B" => PAPER,
            "C" => SCISSORS,
            _ => panic!("oh shit"),
        }
    }

    fn letter_to_score(letter: &str) -> u32 {
        match letter {
            "X" => LOSS,
            "Y" => DRAW,
            "Z" => WIN,
            _ => panic!("oh shit"),
        }
    }

    let mut score: u32 = 0;

    for line in input.split("\n") {
        let (a, b) = line.split_once(" ").unwrap();
        let (a, b) = (letter_to_hand(a), letter_to_score(b));

        let mut round_score = b;

        let hand = match (a, b) {
            (ROCK, WIN) => PAPER,
            (ROCK, LOSS) => SCISSORS,
            (PAPER, WIN) => SCISSORS,
            (PAPER, LOSS) => ROCK,
            (SCISSORS, WIN) => ROCK,
            (SCISSORS, LOSS) => PAPER,
            (hand, DRAW) => hand,
            _ => panic!("oh shit 2"),
        };

        round_score += hand;
        score += round_score;
    }

    return score;
}
