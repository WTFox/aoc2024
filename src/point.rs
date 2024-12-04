use crate::Direction;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Default for Point {
    fn default() -> Self {
        Point::new()
    }
}

impl Point {
    pub fn new() -> Point {
        Point { x: 0, y: 0 }
    }

    pub fn value_at(&self, grid: &[Vec<char>]) -> char {
        grid[self.y as usize][self.x as usize]
    }

    pub fn value_in_direction(&self, direction: Direction, grid: &[Vec<char>]) -> Option<char> {
        let new_point = self.moved(direction);
        if new_point.y < 0 || new_point.y >= grid.len() as i32 {
            return None;
        }
        if new_point.x < 0 || new_point.x >= grid[0].len() as i32 {
            return None;
        }
        Some(new_point.value_at(grid))
    }

    pub fn distance(&self) -> i32 {
        self.x.abs() + self.y.abs()
    }

    pub fn distance_from(&self, other: &Point) -> i32 {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }

    pub fn move_in_direction(&mut self, direction: Direction) {
        match direction {
            Direction::Up => self.y -= 1,
            Direction::UpRight => {
                self.y -= 1;
                self.x += 1;
            }
            Direction::Right => self.x += 1,
            Direction::DownRight => {
                self.y += 1;
                self.x += 1;
            }
            Direction::Down => self.y += 1,
            Direction::DownLeft => {
                self.y += 1;
                self.x -= 1;
            }
            Direction::Left => self.x -= 1,
            Direction::UpLeft => {
                self.y -= 1;
                self.x -= 1;
            }
        }
    }

    pub fn attempt_move(&self, direction: Direction, grid: &[Vec<char>]) -> Option<Point> {
        let new_point = self.moved(direction);
        if new_point.y < 0 || new_point.y >= grid.len() as i32 {
            return None;
        }
        if new_point.x < 0 || new_point.x >= grid[0].len() as i32 {
            return None;
        }
        Some(new_point)
    }

    pub fn moved(&self, direction: Direction) -> Self {
        let mut new_point = *self;
        match direction {
            Direction::Up => new_point.y -= 1,
            Direction::UpRight => {
                new_point.y -= 1;
                new_point.x += 1;
            }
            Direction::Right => new_point.x += 1,
            Direction::DownRight => {
                new_point.y += 1;
                new_point.x += 1;
            }
            Direction::Down => new_point.y += 1,
            Direction::DownLeft => {
                new_point.y += 1;
                new_point.x -= 1;
            }
            Direction::Left => new_point.x -= 1,
            Direction::UpLeft => {
                new_point.y -= 1;
                new_point.x -= 1;
            }
        }
        new_point
    }
}
