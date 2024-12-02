#[allow(unused)]
fn process_input(input: &str) -> Vec<Vec<isize>> {
    input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|x| x.parse().unwrap())
                .collect()
        })
        .collect()
}

pub fn part_one(_input: &str) -> i32 {
    42
}

pub fn part_two(_input: &str) -> i32 {
    42
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "";

    #[test]
    fn test_part_one() {
        let result = part_one(INPUT);
        assert_eq!(result, 42);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(INPUT);
        assert_eq!(result, 42);
    }
}
