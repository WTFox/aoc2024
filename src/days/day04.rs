use crate::Direction;
use crate::Point;

fn process_input(input: &str) -> Vec<Vec<char>> {
    input
        .trim()
        .lines()
        .map(|line| line.chars().collect())
        .collect()
}

fn find_word(grid: &[Vec<char>], x: i32, y: i32, direction: Direction, length: i32) -> String {
    let mut point = Point {
        x: x as i32,
        y: y as i32,
    };
    let mut output = String::from(point.value_at(grid));
    (0..length - 1).for_each(|_i| {
        if let Some(new_point) = point.attempt_move(direction, grid) {
            point = new_point;
            output.push(point.value_at(grid));
        }
    });
    output
}

pub fn part_one(input: &str) -> i32 {
    let looking_for = "XMAS";
    let grid = process_input(input);

    let mut count = 0;
    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            for direction in Direction::iterator() {
                let word = find_word(
                    &grid,
                    x as i32,
                    y as i32,
                    *direction,
                    looking_for.len() as i32,
                );
                if word == looking_for {
                    count += 1;
                }
            }
        }
    }
    count
}

pub fn part_two(input: &str) -> i32 {
    let mut count = 0;
    let grid = process_input(input);

    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            let point = Point {
                x: x as i32,
                y: y as i32,
            };
            if point.value_at(&grid) != 'A' {
                continue;
            }

            let pairs = [
                (
                    point.value_in_direction(Direction::UpLeft, &grid),
                    point.value_in_direction(Direction::DownRight, &grid),
                ),
                (
                    point.value_in_direction(Direction::UpRight, &grid),
                    point.value_in_direction(Direction::DownLeft, &grid),
                ),
            ];

            let mut words = pairs.iter().map(|(left, right)| {
                if Some('M') == *left {
                    format!("{}A{}", left.unwrap_or(' '), right.unwrap_or(' '))
                } else if Some('M') == *right {
                    format!("{}A{}", right.unwrap_or(' '), left.unwrap_or(' '))
                } else {
                    String::new()
                }
            });

            if words.all(|word| word == "MAS") {
                count += 1;
            }
        }
    }

    count
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "\
MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";

    #[test]
    fn test_part_one() {
        let result = part_one(INPUT);
        assert_eq!(result, 18);
    }

    #[test]
    fn test_find_word() {
        let test_cases = [
            (Point { x: 5, y: 0 }, Direction::Right),
            (Point { x: 9, y: 3 }, Direction::Down),
            (Point { x: 6, y: 4 }, Direction::Left),
            (Point { x: 9, y: 9 }, Direction::Up),
            (Point { x: 0, y: 5 }, Direction::UpRight),
            (Point { x: 6, y: 5 }, Direction::UpLeft),
            (Point { x: 9, y: 3 }, Direction::DownLeft),
            (Point { x: 4, y: 0 }, Direction::DownRight),
        ];
        let grid = process_input(INPUT);
        for (point, direction) in test_cases.iter() {
            let point = *point;
            assert_eq!(find_word(&grid, point.x, point.y, *direction, 4), "XMAS");
        }
    }

    #[test]
    fn test_part_two() {
        let result = part_two(INPUT);
        assert_eq!(result, 9);
    }
}
