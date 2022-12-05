use crate::Instruction;
use std::{fmt::Display, str::FromStr};

#[derive(Debug)]
pub struct Ship {
    containers: [Vec<char>; 9],
}

#[derive(Debug, Clone)]
pub struct ShipError;

impl Ship {
    pub fn move_items_one_by_one(&mut self, instruction: &Instruction) {
        for _ in 0..instruction.quantitity {
            let item = self.containers[instruction.from].pop().unwrap();
            self.containers[instruction.to].push(item);
        }
    }

    pub fn move_items_in_bulk(&mut self, instruction: &Instruction) {
        let mut temp: Vec<char> = Vec::new();
        for _ in 0..instruction.quantitity {
            let item = self.containers[instruction.from].pop().unwrap();
            temp.push(item);
        }

        for item in temp.iter().rev() {
            self.containers[instruction.to].push(*item);
        }
    }

    pub fn get_tops(&self) -> Vec<char> {
        return self
            .containers
            .iter()
            .map(|c| *c.last().clone().unwrap())
            .collect();
    }
}

impl FromStr for Ship {
    type Err = ShipError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut containers: [Vec<char>; 9] = Default::default();

        for line in s.lines() {
            let chars: Vec<char> = line.chars().collect();
            for i in 0..9 {
                let char_ix = 4 * i + 1;
                let item = chars[char_ix];
                if item != ' ' {
                    containers[i].insert(0, item);
                }
            }
        }

        return Ok(Ship { containers });
    }
}

impl Display for Ship {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut repr = String::new();
        for (ix, container) in self.containers.iter().enumerate() {
            repr.push_str(&format!(
                "\t{}: [{}]\n",
                ix + 1,
                container
                    .iter()
                    .map(|x| x.to_string())
                    .collect::<Vec<String>>()
                    .join(", ")
            ));
        }

        return write!(f, "Ship {{\n{}}}", repr);
    }
}
