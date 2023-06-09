mod data_parser;
mod game;

use data_parser::DataParser;
use game::Game;
fn main() {
    let rounds = DataParser::get_and_parse_data("input.txt");

    let game = Game::new(rounds);
    game.print();
}
#[cfg(test)]
mod tests {
    use crate::{data_parser::DataParser, game::Game};

    #[test]
    fn test_integration() {
        let rounds = DataParser::get_and_parse_data("input_test.txt");

        let game = Game::new(rounds);
        assert_eq!(game.calculate_game(), 12);
    }
}
