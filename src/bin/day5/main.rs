use std::str::FromStr;

mod instruction;
mod ship;

use instruction::Instruction;
use ship::Ship;
use std::fs;

fn main() {
    let input = fs::read_to_string("./src/bin/day5/input.txt").expect("ooga booga");

    let lines: Vec<&str> = input.lines().collect();
    let ship_contents = &lines[0..8].join("\n");
    let instructions = &lines[10..];

    let mut ship = Ship::from_str(ship_contents.as_str()).unwrap();
    let instructions = instructions
        .iter()
        .map(|x| Instruction::from_str(x).unwrap());

    // part 1
    // for instruction in instructions {
    //     ship.move_items_one_by_one(&instruction);
    // }

    // part 2
    for instruction in instructions {
        ship.move_items_in_bulk(&instruction);
    }

    for item in ship.get_tops() {
        print!("{}", item);
    }
    println!();
}
