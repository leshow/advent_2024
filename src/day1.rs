use std::collections::HashMap;

pub fn run(input: &str) -> u32 {
    let (mut l, mut r): (Vec<u32>, Vec<u32>) = input
        .lines()
        .flat_map(|line| {
            let mut iter = line.split_whitespace().flat_map(|n| n.parse::<u32>());
            Some((iter.next()?, iter.next()?))
        })
        .unzip();
    l.sort();
    r.sort();
    l.into_iter().zip(r).map(|(l, r)| r.abs_diff(l)).sum()
}

pub fn run_2(input: &str) -> u32 {
    let (l, r): (Vec<u32>, Vec<u32>) = input
        .lines()
        .flat_map(|line| {
            let mut iter = line.split_whitespace().flat_map(|n| n.parse::<u32>());
            Some((iter.next()?, iter.next()?))
        })
        .unzip();
    let map = r.into_iter().fold(HashMap::new(), |mut map, n| {
        *map.entry(n).or_insert(0) += 1;
        map
    });

    l.into_iter().map(|n| n * *map.get(&n).unwrap_or(&0)).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(run(include_str!("../data/day1/a.txt")), 1_830_467);
    }

    #[test]
    fn test_2() {
        assert_eq!(run_2(include_str!("../data/day1/a.txt")), 26_674_158);
    }
}
