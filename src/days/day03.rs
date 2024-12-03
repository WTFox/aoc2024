use regex::Regex;

fn process_multiples(input: &str) -> i32 {
    Regex::new(r"mul\((\d+),(\d+)\)")
        .unwrap()
        .captures_iter(input)
        .map(|cap| cap[1].parse::<i32>().unwrap() * cap[2].parse::<i32>().unwrap())
        .sum()
}

pub fn part_one(input: &str) -> i32 {
    process_multiples(input)
}

pub fn part_two(input: &str) -> i32 {
    let regex = Regex::new(r"mul\(([0-9]{1,3}),([0-9]{1,3})\)|don't\(\)|do\(\)").unwrap();
    let mut enabled = true;
    regex
        .captures_iter(input)
        .filter_map(|cap| match &cap[0] {
            "do()" => {
                enabled = true;
                None
            }
            "don't()" => {
                enabled = false;
                None
            }
            _ => {
                if enabled {
                    Some(cap[1].parse::<i32>().unwrap() * cap[2].parse::<i32>().unwrap())
                } else {
                    None
                }
            }
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const PART_ONE_INPUT: &str =
        "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";

    #[test]
    fn test_part_one() {
        let result = part_one(PART_ONE_INPUT);
        assert_eq!(result, 161);
    }
    const PART_TWO_INPUT: &str =
        "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5)";

    #[test]
    fn test_part_two() {
        let result = part_two(PART_TWO_INPUT);
        assert_eq!(result, 48);
    }
}
