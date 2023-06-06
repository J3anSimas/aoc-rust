use data_parser::DataParser;

mod inventories;

mod data_parser;

fn main() {
    let inventories_manager = DataParser::get_and_parse_data("input.txt");

    inventories_manager.print();
}
