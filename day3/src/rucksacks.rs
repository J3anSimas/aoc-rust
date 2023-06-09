const POSSIBLE_ITEMS: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
struct Compartment {
    items: Vec<String>,
}

impl Compartment {
    fn new(items: Vec<String>) -> Self {
        Self { items }
    }
}
struct Rucksack {
    compartments: [Compartment; 2],
}
impl Rucksack {
    fn new(compartments: [Compartment; 2]) -> Self {
        Self { compartments }
    }

    fn get_duplicated_item(&self) -> Option<String> {
        for i in &self.compartments[0].items {
            for j in &self.compartments[1].items {
                if i == j {
                    return Some(i.clone());
                }
            }
        }
        return None;
    }
    pub fn get_duplicated_priority(&self) -> u32 {
        let priority: usize = match self.get_duplicated_item() {
            Some(x) => {
                POSSIBLE_ITEMS
                    .find(x.as_str())
                    .expect("Could not identify item")
                    + 1
            }

            None => 0,
        };
        return priority as u32;
    }
}
struct RucksacksManager {
    rucksacks: Vec<Rucksack>,
}

impl RucksacksManager {
    fn new(rucksacks: Vec<Rucksack>) -> Self {
        Self { rucksacks }
    }
    fn get_sum_priorities(&self) -> u32 {
        let sum: u32 = self
            .rucksacks
            .iter()
            .map(|x| x.get_duplicated_priority())
            .sum();
        return sum;
    }
}
#[cfg(test)]
mod tests {
    use super::{Compartment, Rucksack, RucksacksManager};

    #[test]
    fn test_duplicated_item() {
        let compartments = [
            Compartment::new(vec!["a".to_string(), "b".to_string(), "c".to_string()]),
            Compartment::new(vec!["A".to_string(), "e".to_string(), "c".to_string()]),
        ];
        let rucksack = Rucksack::new(compartments);
        assert_eq!(rucksack.get_duplicated_item(), Some("c".to_string()));
    }
    #[test]
    fn test_duplicated_priority() {
        let compartments = [
            Compartment::new(vec!["A".to_string(), "b".to_string(), "C".to_string()]),
            Compartment::new(vec!["A".to_string(), "e".to_string(), "c".to_string()]),
        ];
        let rucksack = Rucksack::new(compartments);
        assert_eq!(rucksack.get_duplicated_priority(), 27);
    }

    #[test]
    fn test_sum_priorities() {
        let compartments = [
            Compartment::new(vec!["A".to_string(), "b".to_string(), "C".to_string()]),
            Compartment::new(vec!["A".to_string(), "e".to_string(), "c".to_string()]),
        ];
        let rucksack1 = Rucksack::new(compartments);
        let compartments = [
            Compartment::new(vec!["m".to_string(), "e".to_string(), "C".to_string()]),
            Compartment::new(vec!["A".to_string(), "e".to_string(), "c".to_string()]),
        ];
        let rucksack2 = Rucksack::new(compartments);
        let rucksacks_manager = RucksacksManager::new(vec![rucksack2, rucksack1]);
        assert_eq!(rucksacks_manager.get_sum_priorities(), 27 + 5);
    }
}

