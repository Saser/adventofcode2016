use std::str::FromStr;

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
enum Direction {
    North,
    East,
    South,
    West,
}

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
enum Turn {
    Right,
    Left,
}

impl FromStr for Turn {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "R" => Ok(Turn::Right),
            "L" => Ok(Turn::Left),
            _ => Err("invalid turn".to_string()),
        }
    }
}

impl Direction {
    fn turn(&self, turn: &Turn) -> Direction {
        match *turn {
            Turn::Right => self.turn_right(),
            Turn::Left => self.turn_left(),
        }
    }

    fn turn_right(&self) -> Direction {
        match *self {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
        }
    }

    fn turn_left(&self) -> Direction {
        match *self {
            Direction::North => Direction::West,
            Direction::East => Direction::North,
            Direction::South => Direction::East,
            Direction::West => Direction::South,
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
struct Instruction {
    turn: Turn,
    steps: i32,
}


impl FromStr for Instruction {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (turn_str, steps_str) = s.split_at(1);
        let turn = Turn::from_str(turn_str).unwrap();
        let steps = i32::from_str(steps_str).unwrap();
        Ok(Instruction {
            turn: turn,
            steps: steps,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn many_turns(init_dir: &Direction, turns: &[Turn]) -> Direction {
        turns.iter()
            .fold(init_dir.clone(), |acc, turn| acc.turn(turn))
    }

    #[test]
    fn turn_right_then_left() {
        let dir = Direction::North;
        let newdir = many_turns(&dir, &[Turn::Right, Turn::Left]);

        assert_eq!(dir, newdir);
    }

    #[test]
    fn turn_left_then_right() {
        let dir = Direction::North;
        let newdir = many_turns(&dir, &[Turn::Left, Turn::Right]);

        assert_eq!(dir, newdir);
    }

    #[test]
    fn turn_right_four_times() {
        let dir = Direction::East;
        let newdir = many_turns(&dir, &[Turn::Right; 4]);

        assert_eq!(dir, newdir);
    }

    #[test]
    fn turn_left_four_times() {
        let dir = Direction::West;
        let newdir = many_turns(&dir, &[Turn::Left; 4]);

        assert_eq!(dir, newdir);
    }

    #[test]
    fn turn_fromstr() {
        let right = Turn::from_str("R").unwrap();
        let left = Turn::from_str("L").unwrap();
        let invalid = Turn::from_str("invalid");

        assert_eq!(right, Turn::Right);
        assert_eq!(left, Turn::Left);
        assert!(invalid.is_err());
    }

    #[test]
    fn instruction_fromstr() {
        let i1 = Instruction::from_str("L14").unwrap();
        assert_eq!(i1,
                   Instruction {
                       turn: Turn::Left,
                       steps: 14,
                   });

        let i2 = Instruction::from_str("R14").unwrap();
        assert_eq!(i2,
                   Instruction {
                       turn: Turn::Right,
                       steps: 14,
                   });
    }
}
