#[derive(Debug)]
enum Direction {
    North,
    East,
    South,
    West,
}

#[derive(Debug)]
enum Turn {
    Right,
    Left,
}

impl Direction {
    fn turn(&self, turn: Turn) -> Direction {
        match turn {
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

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}
