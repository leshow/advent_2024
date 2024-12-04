use anyhow::Result;

const DIRECTION: [(isize, isize); 8] = [
    (-1, 0),
    (-1, 1),
    (0, 1),
    (1, 1),
    (1, 0),
    (1, -1),
    (0, -1),
    (-1, -1),
];

pub fn run(input: &str) -> u32 {
    let board = input
        .lines()
        .map(|line| line.as_bytes())
        .collect::<Vec<_>>();
    let rows = board[0].len();
    let cols = board.len();
    let mut total = 0;
    for row in 0..rows {
        for col in 0..cols {
            for dir in DIRECTION {
                if check(&board, row, col, dir, b"XMAS") {
                    total += 1;
                }
            }
        }
    }
    total
}

fn check(board: &[&[u8]], i: usize, j: usize, (dx, dy): (isize, isize), s: &[u8]) -> bool {
    // for each letter in `s`, and add the direction to the i, j and compare to current char in s
    s.iter()
        .scan((i as isize, j as isize), |(x, y), &c| {
            let res = board
                .get(*y as usize)
                .and_then(|line| line.get(*x as usize));
            *x += dx;
            *y += dy;
            Some(res.map(|&r| r == c))
        })
        .all(|r| r.unwrap_or(false))
}

pub fn run_2(input: &str) -> u32 {
    let board = input
        .lines()
        .map(|line| line.as_bytes())
        .collect::<Vec<_>>();
    let rows = board[0].len();
    let cols = board.len();
    let mut total = 0;
    for row in 0..rows {
        for col in 0..cols {
            if (check(&board, row + 2, col + 2, (-1, -1), b"MAS")
                || check(&board, row, col, (1, 1), b"MAS"))
                && (check(&board, row + 2, col, (-1, 1), b"MAS")
                    || check(&board, row, col + 2, (1, -1), b"MAS"))
            {
                total += 1;
            }
        }
    }
    total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(run(include_str!("../data/day4/a.txt")), 2483);
    }

    #[test]
    fn test_2() {
        assert_eq!(run_2(include_str!("../data/day4/a.txt")), 2483);
    }
}
