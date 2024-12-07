use itertools::Itertools;
use std::fs;

fn walk(
    m: &[Vec<u8>],
    mut row: usize,
    mut col: usize,
    return_squares: bool,
) -> Option<Vec<(usize, usize)>> {
    // init seen matrix with [false; 4] for each cell
    let mut seen = vec![vec![[false; 4]; m[0].len()]; m.len()];
    let mut d: usize = 0; // direction index

    loop {
        if seen[row][col][d] {
            // detected a loop, return None (no exit found)
            return None;
        }
        seen[row][col][d] = true;

        // directional offsets: up, right, down, left
        let directions: [(isize, isize); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];
        let (drow, dcol) = directions[d];

        // calc new position using isize (negative offsets)
        let new_r = row as isize + drow;
        let new_c = col as isize + dcol;

        // is new position out of bounds?
        if new_r < 0 || new_r >= m.len() as isize || new_c < 0 || new_c >= m[0].len() as isize {
            if !return_squares {
                return Some(Vec::new());
            }
            // collect all visited squares
            let visited = (0..m.len())
                .cartesian_product(0..m[0].len())
                .filter(|&(r, c)| seen[r][c].iter().any(|&b| b))
                .collect();
            return Some(visited);
        }

        // convert back to usize (bounds ok)
        let rr = new_r as usize;
        let cc = new_c as usize;

        if m[rr][cc] == b'#' {
            // hit obstacle; change direction clockwise
            d = (d + 1) % 4;
        } else {
            // move to new position
            row = rr;
            col = cc;
        }
    }
}

fn process_first(input: String) -> usize {
    let grid = input
        .lines()
        .map(|line| line.as_bytes().to_vec())
        .collect::<Vec<_>>();

    let (srow, scol) = (0..grid.len())
        .cartesian_product(0..grid[0].len())
        .find(|&(r, c)| grid[r][c] == b'^')
        .unwrap();

    walk(&grid, srow, scol, true).unwrap().len()
}

fn process_second(input: String) -> usize {
    let mut grid = input
        .lines()
        .map(|line| line.as_bytes().to_vec())
        .collect::<Vec<_>>();

    let (srow, scol) = (0..grid.len())
        .cartesian_product(0..grid[0].len())
        .find(|&(row, col)| grid[row][col] == b'^')
        .unwrap();

    walk(&grid, srow, scol, true)
        .unwrap()
        .iter()
        .filter(|&&(row, col)| {
            grid[row][col] = b'#';
            let ok = walk(&grid, srow, scol, false).is_none();
            grid[row][col] = b'.';
            ok
        })
        .count()
}

fn main() {
    let input = fs::read_to_string("input1.txt").expect("Error reading file");

    let first_r = process_first(input.clone());
    let second_r = process_second(input.clone());

    println!("first: {first_r}");
    println!("second: {second_r}");
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;
    const INPUT: &str = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";

    #[test]
    fn test_first() -> Result<()> {
        assert_eq!(41, process_first(INPUT.to_owned()));
        Ok(())
    }

    #[test]
    fn test_second() -> Result<()> {
        assert_eq!(6, process_second(INPUT.to_owned()));
        Ok(())
    }
}
