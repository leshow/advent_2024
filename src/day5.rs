use std::{cmp::Ordering, collections::HashMap};

struct Rule {
    before: Vec<u32>,
}

pub fn run(input: &str) -> Option<u32> {
    let (fst, snd) = input.split_once("\n\n")?;
    let rules = rules(fst);
    Some(
        snd.lines()
            .flat_map(|line| {
                let res = line
                    .split(',')
                    .flat_map(|n| n.parse::<u32>())
                    .collect::<Vec<_>>();
                let middle = res[res.len() / 2];
                for i in 0..res.len() {
                    let after = &res[i + 1..res.len()];
                    if rules[&res[i]].before.iter().any(|n| after.contains(n)) {
                        return None;
                    }
                }
                Some(middle)
            })
            .sum(),
    )
}

fn rules(fst: &str) -> HashMap<u32, Rule> {
    fst.lines()
        .flat_map(|line| {
            let (a, b) = line.split_once('|')?;
            let a = a.parse::<u32>().ok()?;
            let b = b.parse::<u32>().ok()?;
            Some((a, b))
        })
        .fold(HashMap::<u32, Rule>::new(), |mut map, (before, after)| {
            map.entry(after)
                .or_insert(Rule { before: Vec::new() })
                .before
                .push(before);
            map
        })
}

pub fn run_2(input: &str) -> Option<u32> {
    let (fst, snd) = input.split_once("\n\n")?;
    let rules = rules(fst);
    Some(
        snd.lines()
            .flat_map(|line| {
                let res = line
                    .split(',')
                    .flat_map(|n| n.parse::<u32>())
                    .collect::<Vec<_>>();
                for i in 0..res.len() {
                    let after = &res[i + 1..res.len()];
                    if rules[&res[i]].before.iter().any(|n| after.contains(n)) {
                        return Some(res);
                    }
                }
                None
            })
            .map(|mut line| {
                line.sort_by(|a, b| {
                    if rules[a].before.contains(b) {
                        Ordering::Less
                    } else {
                        Ordering::Greater
                    }
                });
                line[line.len() / 2]
            })
            .sum(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(run(include_str!("../data/day5/a.txt")), Some(5713));
    }

    #[test]
    fn test_2() {
        assert_eq!(run_2(include_str!("../data/day5/a.txt")), Some(5180));
    }
}
