
#[derive(Eq, Hash, PartialEq)]
pub enum Strategy {
    Win,
    Draw,
    Lose,
}
#[derive(PartialEq, Hash, Eq, Debug, Clone, Copy)]
pub enum Shape {
    Rock,
    Paper,
    Scissor,
}

impl Shape {
    pub fn get_shape_strategy(opponent: Shape, strategy: Strategy) -> Shape {
        match  opponent {
            Shape::Rock => match strategy {
                Strategy::Win => Shape::Paper,
                Strategy::Draw => opponent,
                Strategy::Lose => Shape::Scissor,
            },
            Shape::Paper => match strategy {
                Strategy::Win => Shape::Scissor,
                Strategy::Draw => opponent,
                Strategy::Lose => Shape::Rock,
            },
            Shape::Scissor => match strategy {
                Strategy::Win => Shape::Rock,
                Strategy::Draw => opponent,
                Strategy::Lose => Shape::Paper,
            },
        }
    }
    pub fn get_shape_point(&self) -> u32 {
        let points = match self {
            Shape::Rock => 1,
            Shape::Paper => 2,
            Shape::Scissor => 3,
        };
        return points;
    }
    pub fn beat_it(&self, other: Shape) -> bool {
        if *self == Shape::Rock && other == Shape::Scissor {
            return true;
        } else if *self == Shape::Paper && other == Shape::Rock {
            return true;
        } else if *self == Shape::Scissor && other == Shape::Paper {
            return true;
        } else {
            return false;
        }
    }
}

pub struct Round {
    points: u32,
}

impl Round {
    pub fn new(opponent: Shape, me: Shape) -> Self {
        let mut points = me.get_shape_point();
        if me == opponent {
            points += 3;
        } else if me.beat_it(opponent) {
            points += 6;
        }

        Self { points }
    }
    pub fn get_points(&self) -> u32 {
        return self.points;
    }
}
pub struct Game {
    rounds: Vec<Round>,
}

impl Game {
    pub fn new(rounds: Vec<Round>) -> Self {
        Self { rounds }
    }
    pub fn calculate_game(&self) -> u32 {
        let sum: u32 = self.rounds.iter().map(|x| x.get_points()).sum();
        return sum;
    }
    pub fn print(&self) {
        self.rounds.iter().for_each(|x| println!("{}", x.get_points()));
        println!("================================================");
        println!("Total points: {}", self.calculate_game());
    }
}

#[cfg(test)]
mod tests {
    use super::{Round, Shape};

    #[test]
    fn test_shape_points() {
        let rock = Shape::Rock;
        let paper = Shape::Paper;
        let scissor = Shape::Scissor;

        assert_eq!(rock.get_shape_point(), 1);
        assert_eq!(paper.get_shape_point(), 2);
        assert_eq!(scissor.get_shape_point(), 3);
    }

    #[test]
    fn test_calculate_round() {
        let rock = Shape::Rock;
        let paper = Shape::Paper;
        let scissor = Shape::Scissor;

        let round = Round::new(rock, paper);
        assert_eq!(round.get_points(), 8);
        let round = Round::new(rock, scissor);
        assert_eq!(round.get_points(), 3);
        let round = Round::new(rock, rock);
        assert_eq!(round.get_points(), 4);

        let round = Round::new(paper, paper);
        assert_eq!(round.get_points(), 5);
        let round = Round::new(paper, scissor);
        assert_eq!(round.get_points(), 9);
        let round = Round::new(paper, rock);
        assert_eq!(round.get_points(), 1);

        let round = Round::new(scissor, paper);
        assert_eq!(round.get_points(), 2);
        let round = Round::new(scissor, scissor);
        assert_eq!(round.get_points(), 6);
        let round = Round::new(scissor, rock);
        assert_eq!(round.get_points(), 7);
    }
}
