use std::str::FromStr;

#[derive(Debug)]
pub struct Instruction {
    pub quantitity: usize,
    pub from: usize,
    pub to: usize,
}

#[derive(Debug, Clone)]
pub struct InstructionErr;

impl FromStr for Instruction {
    type Err = InstructionErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let normalized = s
            .chars()
            .filter(|x| x.is_numeric() || *x == ' ')
            .collect::<String>();

        let mut values = normalized
            .trim()
            .split("  ")
            .map(|x| x.parse::<usize>().unwrap());

        let quantitity = values.next().unwrap();
        let from = values.next().unwrap() - 1;
        let to = values.next().unwrap() - 1;

        return Ok(Instruction {
            quantitity,
            from,
            to,
        });
    }
}
