pub struct ElfInventory {
    calories: Vec<u64>,
}

impl ElfInventory {
    pub fn new(calories: Vec<u64>) -> Self {
        Self { calories }
    }
    pub fn get_total_calories(&self) -> u64 {
        let sum: u64 = self.calories.iter().sum();
        return sum;
    }
}

pub struct InventoriesManager {
    inventories: Vec<ElfInventory>,
}

impl InventoriesManager {
    pub fn new(inventories: Vec<ElfInventory>) -> Self {
        Self { inventories }
    }
    pub fn get_most_calories_inventory(&self) -> u64 {
        let mut max = 0;
        for el in &self.inventories {
            if el.get_total_calories() >= max {
                max = el.get_total_calories();
            }
        }
        return max;
    }
    fn get_total_calories(&self) -> u64 {
        let sum: u64 = self
            .inventories
            .iter()
            .map(|x| x.get_total_calories())
            .sum();
        return sum;
    }
    pub fn get_top_three_sum(&self) -> u64 {
        let mut top = [0, 0, 0];

        for el in &self.inventories {
            let current_value = el.get_total_calories();
            if current_value > top[0] {
                top = [current_value, top[0], top[1]];
            } else if current_value > top[1] {
                top = [top[0], current_value, top[1]];
            } else if current_value > top[2] {
                top = [top[0], top[1], current_value];
            }
        }
        return top.into_iter().sum();
    }
    pub fn print(&self) -> () {
        println!(
            "Most calories inventory: {}",
            &self.get_most_calories_inventory()
        );
        println!("Total calories: {}", &self.get_total_calories());
        println!("Top 3 sum calories: {}", &self.get_top_three_sum());
    }
}
