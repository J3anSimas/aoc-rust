use crate::rucksacks::{Compartment, Rucksack, RucksacksManager};
use std::fs;

pub struct InputParser {}

impl InputParser {
    pub fn execute(path: &str) -> RucksacksManager {
        let input: String = String::from_utf8(fs::read(path).expect("Could not find the file"))
            .expect("Could not parse the file data");
        let lines: Vec<&str> = input.split_terminator("\n").collect();
        let rucksacks: Vec<Rucksack> = lines
            .iter()
            .map(|line| {
                let line = line.trim();
                let items: Vec<String> =
                    line.split_terminator("").map(|x| String::from(x)).collect();
                let (first_half, last_half) = items.split_at(items.len() / 2 + 1 + 1);
                let compartments: [Compartment; 2] = [
                    Compartment::new(first_half.to_owned()[1..][1..].to_vec()),
                    Compartment::new(last_half.to_owned()),
                ];
                return Rucksack::new(compartments);
            })
            .collect();
        return RucksacksManager::new(rucksacks);
    }
}
