use super::inventories::{ElfInventory, InventoriesManager};
use std::fs::read;

pub struct DataParser {}
impl DataParser {
    fn get_data(path: &str) -> String {
        let raw_data = if let Ok(x) = read(path) {
            x
        } else {
            panic!("Could not read or find any file");
        };
        if let Ok(x) = String::from_utf8(raw_data) {
            x
        } else {
            panic!("Could not convert data passed through file to string");
        }
    }

    fn parse_data(data: String) -> InventoriesManager {
        let mut inventories: Vec<ElfInventory> = vec![];
        let all_cal: Vec<&str> = data.split("\n\n").collect();
        for i in all_cal {
            let mut temp_vec: Vec<u64> = vec![];
            i.split("\n").for_each(|x| {
                if x != "" {
                    temp_vec.push(x.parse().unwrap());
                }
            });
            inventories.push(ElfInventory::new(temp_vec));
        }
        return InventoriesManager::new(inventories);
    }
    pub fn get_and_parse_data(data: &str) -> InventoriesManager {
        return DataParser::parse_data(DataParser::get_data(data));
    }
}
