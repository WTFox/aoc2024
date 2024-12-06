#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum Direction {
    Up,
    UpRight,
    Right,
    DownRight,
    Down,
    DownLeft,
    Left,
    UpLeft,
}

impl TryFrom<char> for Direction {
    type Error = ();

    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            'U' => Ok(Direction::Up),
            'D' => Ok(Direction::Down),
            'L' => Ok(Direction::Left),
            'R' => Ok(Direction::Right),
            '^' => Ok(Direction::Up),
            'v' => Ok(Direction::Down),
            '<' => Ok(Direction::Left),
            '>' => Ok(Direction::Right),
            _ => Err(()),
        }
    }
}

impl Direction {
    pub fn iterator() -> std::slice::Iter<'static, Direction> {
        static DIRECTIONS: [Direction; 8] = [
            Direction::Up,
            Direction::UpRight,
            Direction::Right,
            Direction::DownRight,
            Direction::Down,
            Direction::DownLeft,
            Direction::Left,
            Direction::UpLeft,
        ];

        DIRECTIONS.iter()
    }

    pub fn turn(&self, turn: char) -> Self {
        match (self, turn) {
            (Direction::Up, 'R') => Direction::Right,
            (Direction::Right, 'R') => Direction::Down,
            (Direction::Down, 'R') => Direction::Left,
            (Direction::Left, 'R') => Direction::Up,
            (Direction::Up, 'L') => Direction::Left,
            (Direction::Right, 'L') => Direction::Up,
            (Direction::Down, 'L') => Direction::Right,
            (Direction::Left, 'L') => Direction::Down,
            _ => *self,
        }
    }
}
