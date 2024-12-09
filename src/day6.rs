#[derive(Copy, Clone, PartialEq, Eq, Debug)]
enum P {
    Blank,
    Wall,
    Start,
    Used,
}

const UP: (i32, i32) = (1, 0);
const DOWN: (i32, i32) = (-1, 0);
const LEFT: (i32, i32) = (0, -1);
const RIGHT: (i32, i32) = (0, 1);

const DIRS: [(i32, i32); 4] = [DOWN, RIGHT, UP, LEFT];

pub fn run(input: &str) -> Option<usize> {
    let mut pos = (0, 0);
    let mut grid = input
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
    let mut count = 0;
    for dir in DIRS.iter().cycle() {
        if advance(&mut grid, &mut count, &mut pos, *dir) {
            return Some(count);
        }
    }
    None
}
fn print(grid: &[Vec<P>]) {
    for line in grid {
        println!("{:?}", line);
    }
}

fn advance(
    grid: &mut [Vec<P>],
    count: &mut usize,
    (x, y): &mut (i32, i32),
    (dx, dy): (i32, i32),
) -> bool {
    let cols = grid[0].len() as i32;
    let rows = grid.len() as i32;

    while *x < rows && *x >= 0 && *y < cols && *y >= 0 {
        if let P::Wall = grid[*x as usize][*y as usize] {
            // reverse one and exit
            *x += -dx;
            *y += -dy;
            return false;
        }
        // count unique
        if !matches!(grid[*x as usize][*y as usize], P::Used) {
            *count += 1;
        }
        grid[*x as usize][*y as usize] = P::Used;

        *x += dx;
        *y += dy;
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(run(include_str!("../data/day6/a.txt")), Some(5067));
    }
}
