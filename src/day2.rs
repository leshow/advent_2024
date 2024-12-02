pub fn run(input: &str) -> usize {
    input
        .lines()
        .filter(|line| {
            let nums = line
                .split_whitespace()
                .flat_map(|n| n.parse::<u32>())
                .collect::<Vec<_>>();
            levels(&nums)
        })
        .count()
}

fn levels(nums: &[u32]) -> bool {
    let direction = nums[1] > nums[0];
    nums.windows(2).all(|w| {
        let &[a, b] = w else {
            return false;
        };
        let dist = a.abs_diff(b);
        (b > a) == direction && (1..=3).contains(&dist)
    })
}

pub fn run_2(input: &str) -> usize {
    input
        .lines()
        .filter(|line| {
            let nums = line
                .split_whitespace()
                .flat_map(|n| n.parse::<u32>())
                .collect::<Vec<_>>();
            (0..nums.len()).any(|skip| {
                let mut nums = nums.clone();
                nums.remove(skip);
                levels(&nums)
            })
        })
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(run(include_str!("../data/day2/a.txt")), 631);
    }

    #[test]
    fn test_2() {
        assert_eq!(run_2(include_str!("../data/day2/a.txt")), 665);
    }
}
