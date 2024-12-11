use std::{collections::HashSet, fmt::Debug};

#[derive(Copy, Clone, PartialEq, Eq)]
enum P {
    Blank,
    Wall,
    Start,
    Used,
}

impl Debug for P {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Blank => write!(f, "."),
            Self::Wall => write!(f, "#"),
            Self::Start => write!(f, "^"),
            Self::Used => write!(f, "X"),
        }
    }
}

const UP: (i32, i32) = (1, 0);
const DOWN: (i32, i32) = (-1, 0);
const LEFT: (i32, i32) = (0, -1);
const RIGHT: (i32, i32) = (0, 1);

const DIRS: [(i32, i32); 4] = [DOWN, RIGHT, UP, LEFT];

pub fn run(input: &str) -> Option<usize> {
    let (mut grid, mut pos) = parse(input);
    let mut positions = HashSet::new();
    positions.insert(pos);
    grid[pos.0 as usize][pos.1 as usize] = P::Used;

    for dir in DIRS.iter().cycle() {
        if advance(&mut grid, &mut positions, &mut pos, *dir) {
            print(&grid);
            println!();
            return Some(positions.len());
        }
    }
    None
}

fn print(grid: &[Vec<P>]) {
    use std::fmt::Write;
    for line in grid {
        println!(
            "{}",
            line.iter().fold(String::new(), |mut out, b| {
                let _ = write!(out, "{b:?}");
                out
            })
        );
    }
}

fn parse(input: &str) -> (Vec<Vec<P>>, (i32, i32)) {
    let mut pos = (0, 0);
    let grid = input
        .lines()
        .enumerate()
        .map(|(x, line)| {
            line.as_bytes()
                .iter()
                .enumerate()
                .map(|(y, &c)| match c {
                    b'#' => P::Wall,
                    b'^' => {
                        pos = (x as i32, y as i32);
                        P::Start
                    }
                    _ => P::Blank,
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    (grid, pos)
}

pub fn run_2(input: &str) -> Option<usize> {
    let (mut grid, (mut x, mut y)) = parse(input);
    let mut positions = HashSet::new();
    let mut dirs = DIRS.iter().cycle();

    positions.insert((x, y));
    grid[x as usize][y as usize] = P::Used;
    let (mut dx, mut dy) = *dirs.next()?;

    loop {
        x += dx;
        y += dy;
        match grid
            .get_mut(x as usize)
            .and_then(|line| line.get_mut(y as usize))
        {
            Some(P::Wall) => {
                x += -dx;
                y += -dy;
                (dx, dy) = *dirs.next()?;
                // reverse one and exit
            }
            Some(p) => {
                *p = P::Used;
                positions.insert((x, y));
            }
            None => {
                print(&grid);
                println!();
                return Some(positions.len());
            }
        }
    }
}

fn advance(
    grid: &mut [Vec<P>],
    positions: &mut HashSet<(i32, i32)>,
    (x, y): &mut (i32, i32),
    (dx, dy): (i32, i32),
) -> bool {
    loop {
        let next = (*x + dx, *y + dy);
        match grid
            .get_mut(next.0 as usize)
            .and_then(|line| line.get_mut(next.1 as usize))
        {
            Some(P::Wall) => {
                return false;
            }
            Some(p) => {
                *p = P::Used;
                (*x, *y) = next;
                positions.insert(next);
            }
            None => {
                return true;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(run(include_str!("../data/day6/a.txt")), Some(5067));
    }

    #[test]
    fn test_sample() {
        assert_eq!(run(include_str!("../data/day6/sample.txt")), Some(41));
    }

    #[test]
    fn test_2() {
        assert_eq!(run_2(include_str!("../data/day6/sample.txt")), Some(41));
    }
}
