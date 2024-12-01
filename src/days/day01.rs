pub fn part_one(input: &str) -> i32 {
    let (mut left, mut right): (Vec<i32>, Vec<i32>) = input
        .lines()
        .map(|line| {
            if line.is_empty() {
                return (0, 0);
            }
            let mut nums = line.split_whitespace().map(|n| n.parse::<i32>().unwrap());
            (nums.next().unwrap(), nums.next().unwrap())
        })
        .unzip();

    left.sort();
    right.sort();

    left.iter()
        .zip(right.iter())
        .map(|(l, r)| (r - l).abs())
        .sum()
}

pub fn part_two(_input: &str) -> i32 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    const input: &str = "3   4
4   3
2   5
1   3
3   9
3   3";

    #[test]
    fn test_part_one_example_one() {
        let result = part_one(input);
        assert_eq!(result, 11);
    }
}
