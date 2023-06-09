use crate::game::{Round, Shape, Strategy};

use std::fs::read;

pub struct DataParser {}

impl DataParser {
    fn get_data(path: &str) -> String {
        let data = String::from_utf8(
            read(path).expect("Could not find file or it wasn't possible to read it"),
        )
        .expect("Could not parse the content of the file to UTF-8");
        return data;
    }
    fn parse_strategy(char: &str) -> Strategy {
        return match char {
            "X" => Strategy::Lose,
            "Y" => Strategy::Draw,
            "Z" => Strategy::Win,
            _ => panic!("Could not identify token {}", char),
        };
    }

    fn parse_opponent_shape(char: &str) -> Shape {
        let result = match char {
            "A" => Shape::Rock,
            "B" => Shape::Paper,
            "C" => Shape::Scissor,
            _ => {
                panic!("Could not identify token {}", char);
            }
        };
        return result;
    }

    fn parse_data(data: String) -> Vec<Round> {
        let rows: Vec<&str> = data.split_terminator("\n").collect();
        let mut rounds: Vec<Round> = Vec::new();

        for row in rows {
            let columns: Vec<&str> = row.split_terminator(" ").collect();
            let opponent_shape: Shape = DataParser::parse_opponent_shape(
                columns.get(0).expect("Error trying to parse data input"),
            );
            let strategy: Strategy = DataParser::parse_strategy(
                columns.get(1).expect("Error trying to parse data input"),
            );
            let my_shape = Shape::get_shape_strategy(opponent_shape, strategy);
            rounds.push(Round::new(opponent_shape, my_shape))
        }

        return rounds;
    }
    pub fn get_and_parse_data(path: &str) -> Vec<Round> {
        return DataParser::parse_data(DataParser::get_data(path));
    }
}
#[cfg(test)]
mod tests {
    use super::{Shape, DataParser};
    #[test]
    fn test_parse_shape() {
        assert_eq!(DataParser::parse_opponent_shape("A"), Shape::Rock);
        assert_eq!(DataParser::parse_opponent_shape("B"), Shape::Paper);
        assert_eq!(DataParser::parse_opponent_shape("C"), Shape::Scissor);
    }

    #[test]
    #[should_panic]
    fn test_invalid_token() {
        DataParser::parse_opponent_shape("X");
    }
}
