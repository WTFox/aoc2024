use crate::Direction;
use crate::Point;

fn process_input(input: &str) -> Vec<Vec<char>> {
    input
        .trim()
        .lines()
        .map(|line| line.chars().collect())
        .collect()
}

fn gather_word_in_direction(
    grid: &[Vec<char>],
    x: i32,
    y: i32,
    direction: Direction,
    length: i32,
) -> String {
    let mut point = Point { x, y };
    let mut output = String::from("");
    for _ in 0..length {
        output.push(point.value_at(grid));
        if let Some(new_point) = point.attempt_move(direction, grid) {
            point = new_point;
        }
    }
    output
}

pub fn part_one(input: &str) -> i32 {
    let looking_for = "XMAS";
    let grid = process_input(input);

    let mut count = 0;
    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            count += Direction::iterator()
                .map(|direction| {
                    gather_word_in_direction(
                        &grid,
                        x as i32,
                        y as i32,
                        *direction,
                        looking_for.len() as i32,
                    )
                })
                .filter(|word| word == looking_for)
                .count()
        }
    }
    count as i32
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

            let crosswords = [
                (
                    point.value_in_direction(Direction::UpLeft, &grid),
                    point.value_in_direction(Direction::DownRight, &grid),
                ),
                (
                    point.value_in_direction(Direction::UpRight, &grid),
                    point.value_in_direction(Direction::DownLeft, &grid),
                ),
            ]
            .map(|(left, right)| {
                if Some('M') == left {
                    format!("{}A{}", left.unwrap_or(' '), right.unwrap_or(' '))
                } else {
                    format!("{}A{}", right.unwrap_or(' '), left.unwrap_or(' '))
                }
            });
            count += crosswords.iter().all(|word| word == "MAS") as i32;
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
    fn test_part_two() {
        let result = part_two(INPUT);
        assert_eq!(result, 9);
    }

    #[test]
    fn test_find_word() {
        let test_cases = [
            ((5, 0), Direction::Right),
            ((9, 3), Direction::Down),
            ((6, 4), Direction::Left),
            ((9, 9), Direction::Up),
            ((0, 5), Direction::UpRight),
            ((6, 5), Direction::UpLeft),
            ((9, 3), Direction::DownLeft),
            ((4, 0), Direction::DownRight),
        ];
        let grid = process_input(INPUT);
        for ((x, y), direction) in test_cases.iter() {
            assert_eq!(
                gather_word_in_direction(&grid, *x, *y, *direction, 4),
                "XMAS"
            );
        }
    }
}
