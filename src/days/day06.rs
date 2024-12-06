use crate::{Direction, Point};

use std::collections::HashMap;

#[derive(Debug)]
struct Guard {
    pos: Point,
    dir: Direction,
}

fn find_guard(input: &[Vec<char>]) -> Option<Guard> {
    for (y, _) in input.iter().enumerate() {
        for (x, item) in input[y].iter().enumerate() {
            if let Ok(dir) = Direction::try_from(*item) {
                return Some(Guard {
                    dir,
                    pos: Point {
                        x: x as i32,
                        y: y as i32,
                    },
                });
            }
        }
    }
    None
}

fn process_input(input: &str) -> Vec<Vec<char>> {
    input
        .trim()
        .lines()
        .map(|line| line.chars().collect())
        .collect()
}

pub fn part_one(input: &str) -> i32 {
    let grid = process_input(input);
    let mut guard = find_guard(&grid).expect("didn't find guard");
    let mut visited: HashMap<Point, i32> = HashMap::new();
    *visited.entry(guard.pos).or_insert(0) += 1;
    while let Some(movement) = guard.pos.attempt_move(guard.dir, &grid) {
        guard.pos = movement;
        *visited.entry(guard.pos).or_insert(0) += 1;
        if let Some(value_in_next_position) = guard.pos.value_in_direction(guard.dir, &grid) {
            if value_in_next_position == '#' {
                guard.dir = guard.dir.turn('R');
            }
        }
    }
    visited.keys().count() as i32
}

fn simulate(guard: &mut Guard, grid: &[Vec<char>]) -> bool {
    let mut visited = HashMap::new();
    let mut steps = 0;

    visited.insert((guard.pos, guard.dir), steps);
    while let Some(movement) = guard.pos.attempt_move(guard.dir, grid) {
        steps += 1;
        guard.pos = movement;
        if visited.contains_key(&(guard.pos, guard.dir)) {
            return true;
        }

        visited.insert((guard.pos, guard.dir), steps);
        if let Some(value_in_next_position) = guard.pos.value_in_direction(guard.dir, grid) {
            if value_in_next_position == '#' || value_in_next_position == 'O' {
                guard.dir = guard.dir.turn('R');
            }
        }
    }
    false
}

pub fn part_two(input: &str) -> i32 {
    let grid = process_input(input);

    let mut count = 0;
    for (y, row) in grid.iter().enumerate() {
        for (x, &item) in row.iter().enumerate() {
            if item == '.' {
                let mut new_grid = grid.clone();
                new_grid[y][x] = 'O';

                if let Some(mut guard) = find_guard(&new_grid) {
                    if simulate(&mut guard, &new_grid) {
                        count += 1;
                    }
                }
            }
        }
    }
    count
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "
....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";

    #[test]
    fn test_part_one() {
        let result = part_one(INPUT);
        assert_eq!(result, 41);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(INPUT);
        assert_eq!(result, 6);
    }
}
