use input_parser::InputParser;
fn main() {
    let rucksack_manager = InputParser::execute("input.txt");
    println!("{}", rucksack_manager.get_sum_priorities());
}

#[cfg(test)]
mod tests {
    use crate::input_parser::InputParser;

    #[test]
    fn integration_test() {
        let rucksack_manager = InputParser::execute("input_test.txt");
        rucksack_manager.print();
        assert_eq!(rucksack_manager.get_sum_priorities(), 157)
    }
    #[test]
    fn integration_badge_test(){
        let rucksack_manager = InputParser::execute("input_test.txt");
        rucksack_manager.print();
        assert_eq!(rucksack_manager.get_sum_badge_priority(), 70);
    }
}

mod rucksacks;
mod input_parser;
