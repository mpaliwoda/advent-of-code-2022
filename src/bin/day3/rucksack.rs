use std::collections::HashSet;
use std::str::FromStr;

pub struct Rucksack {
    left: HashSet<char>,
    right: HashSet<char>,
}

#[derive(Debug, Clone)]
pub struct RucksackErr;

impl FromStr for Rucksack {
    type Err = RucksackErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let middle = s.len() / 2;
        let (left, right) = s.split_at(middle);

        return Ok(Rucksack {
            left: HashSet::from_iter(left.chars()),
            right: HashSet::from_iter(right.chars()),
        });
    }
}

impl Rucksack {
    pub fn in_both_compartments(&self) -> Vec<&char> {
        return self.left.intersection(&self.right).collect();
    }

    pub fn common_items(rucksacks: &Vec<Rucksack>) -> HashSet<&char> {
        return rucksacks
            .iter()
            .map(Rucksack::all_items)
            .reduce(|acc, item| acc.intersection(&item).map(|x| *x).collect())
            .unwrap();
    }

    fn all_items(&self) -> HashSet<&char> {
        return self.left.iter().chain(&self.right).collect();
    }
}
