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
        assert!(rucksacks.len() > 0);

        let mut contents = rucksacks.iter().map(Rucksack::all_items);
        let mut result = contents.next().unwrap();

        for rucksack in contents {
            result = result.intersection(&rucksack).map(|x| *x).collect();
        }

        return result;
    }

    fn all_items(&self) -> HashSet<&char> {
        let mut items = HashSet::new();

        for item in self.left.iter().chain(&self.right) {
            items.insert(item);
        }

        return items;
    }
}
