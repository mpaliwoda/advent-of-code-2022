use std::fs;

const MAX_CYCLE_TO_MONITOR: i32 = 220;
const SCREEN_WIDTH: i32 = 40;

fn main() {
    let input = fs::read_to_string("./src/bin/day10/input.txt")
        .expect("the prophecy has not yet been fulfilled");

    println!("Part 1: {}", part_1(&input));

    let screen = part_2(&input);
    for row in screen {
        for ch in row {
            print!("{}", ch);
        }

        println!();
    }
}

fn part_1(input: &String) -> i32 {
    let mut register = 1;
    let mut current_cycle = 1;
    let mut cycle_to_monitor = 20;

    let mut sum_of_monitored_cycles = 0;

    for line in input.lines() {
        let (cycles_to_perform, addx) = match line {
            line if line == "noop" => (1, 0),
            line if line.starts_with("addx") => {
                let (_, amount) = line.split_once(' ').unwrap();
                let amount: i32 = amount.parse().unwrap();
                (2, amount)
            }
            _ => unreachable!("HOW DID YOU DO THAT EXPLAIN YOURSELF"),
        };

        for _ in 0..cycles_to_perform {
            if current_cycle == cycle_to_monitor {
                sum_of_monitored_cycles += register * current_cycle;
                if current_cycle < MAX_CYCLE_TO_MONITOR {
                    cycle_to_monitor += 40;
                }
            }

            current_cycle += 1;
        }

        register += addx;
    }

    return sum_of_monitored_cycles;
}

fn part_2(input: &String) -> [[char; 40]; 6] {
    let mut screen = [[' '; 40]; 6];

    let mut register: i32 = 1;
    let mut current_cycle = 1;

    let mut current_screen_line = 0;
    let mut current_pixel = 0;

    for line in input.lines() {
        let (cycles_to_perform, addx) = match line {
            line if line == "noop" => (1, 0),
            line if line.starts_with("addx") => {
                let (_, amount) = line.split_once(' ').unwrap();
                let amount: i32 = amount.parse().unwrap();
                (2, amount)
            }
            _ => unreachable!("HOW DID YOU DO THAT EXPLAIN YOURSELF"),
        };

        for _ in 0..cycles_to_perform {
            if current_cycle % SCREEN_WIDTH == 0 {
                current_screen_line += 1;

                if current_screen_line == 6 {
                    // i just... ???... how????
                    break;
                }
            }

            let current_char: char = if register.abs_diff(current_pixel) <= 1 {
                '#'
            } else {
                '.'
            };

            screen[current_screen_line as usize][current_pixel as usize] = current_char;

            current_pixel = current_cycle - (SCREEN_WIDTH * current_screen_line);
            current_cycle += 1;
        }

        register += addx;
    }

    return screen;
}
