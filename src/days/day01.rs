use std::collections::HashMap;

fn process_input(input: &str) -> Vec<(i32, i32)> {
    input
        .lines()
        .map(|line| {
            if line.is_empty() {
                return (0, 0);
            }
            let mut nums = line.split_whitespace().map(|n| n.parse::<i32>().unwrap());
            (nums.next().unwrap(), nums.next().unwrap())
        })
        .collect()
}

pub fn part_one(input: &str) -> i32 {
    let (mut left, mut right): (Vec<i32>, Vec<i32>) = process_input(input).iter().cloned().unzip();
    left.sort();
    right.sort();

    left.iter()
        .zip(right.iter())
        .map(|(l, r)| (r - l).abs())
        .sum()
}

pub fn part_two(input: &str) -> i32 {
    let (mut left, mut right): (Vec<i32>, Vec<i32>) = process_input(input).iter().cloned().unzip();
    let mut map: HashMap<i32, i32> = HashMap::new();
    left.sort();
    right.sort();

    left.iter()
        .map(|l| {
            if let Some(v) = map.get(l) {
                return *v;
            }
            let count = right.iter().filter(|&r| r == l).count();
            let product = *l * (count as i32);
            map.insert(*l, product);
            product
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "3   4
4   3
2   5
1   3
3   9
3   3";

    #[test]
    fn test_part_one() {
        let result = part_one(INPUT);
        assert_eq!(result, 11);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(INPUT);
        assert_eq!(result, 31);
    }
}
