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

fn is_safe(report: &[isize]) -> bool {
    let mut prev_diff = report[1] - report[0];

    for i in 1..report.len() {
        let diff = report[i] - report[i - 1];
        if diff.abs() < 1 || diff.abs() > 3 || (diff.signum() != prev_diff.signum()) {
            return false;
        }
        prev_diff = diff;
    }

    true
}

pub fn part_one(input: &str) -> i32 {
    process_input(input).iter().filter(|b| is_safe(b)).count() as i32
}

pub fn part_two(input: &str) -> i32 {
    let mut safe = 0;
    for report in process_input(input).iter() {
        if is_safe(report) {
            safe += 1;
        } else {
            for idx in 0..report.len() {
                let mut sample = report.clone();
                sample.remove(idx);
                if is_safe(&sample) {
                    safe += 1;
                    break;
                }
            }
        }
    }

    safe
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";

    #[test]
    fn test_part_one() {
        let result = part_one(INPUT);
        assert_eq!(result, 2);
    }

    #[test]
    fn test_report_is_safe() {
        assert!(!is_safe(&[1, 2, 8, 9, 11, 12]));
        assert!(is_safe(&[72, 74, 76, 77, 78, 79, 81, 82]));
        assert!(is_safe(&[7, 6, 4, 2, 1]));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(INPUT);
        assert_eq!(result, 4);
    }
}
