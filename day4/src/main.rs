use std::fs;

fn get_char_at_pos(grid: &[&[u8]], row: isize, col: isize) -> u8 {
    // get the character from grid
    // no bounds checking, so when out of bounds: return space so it doesn't match
    // any character of the word

    let the_char = *grid
        .get(row as usize)
        .and_then(|row: &&[u8]| row.get(col as usize))
        .unwrap_or(&b' ');

    the_char
}

fn find_xmas(grid: &[&[u8]], row: isize, col: isize) -> usize {
    let mut xmasses = 0;

    // look around 360
    for (d_row, d_col) in [
        (0, -1),
        (-1, 0),
        (0, 1),
        (1, 0),
        (1, 1),
        (-1, 1),
        (1, -1),
        (-1, -1),
    ] {
        // check for the xmas word length
        let mut oks: Vec<bool> = vec![];

        for i in 1..4 {
            let rr = row + d_row * i;
            let cc = col + d_col * i;
            if rr >= 0 && cc >= 0 {
                // ok if the char fits in its slot in XMAS
                oks.push(get_char_at_pos(grid, rr, cc) == b"XMAS"[i as usize])
            } else {
                oks.push(false)
            }
        }

        if oks.iter().all(|&x| x) {
            xmasses += 1;
        }
    }
    xmasses
}

fn find_mas(grid: &[&[u8]], row: isize, col: isize) -> usize {
    // need an M and S around the A, M or S in X shape
    // left top to right bottom

    let mas1 = [
        get_char_at_pos(grid, row - 1, col - 1),
        get_char_at_pos(grid, row + 1, col + 1),
    ];
    let mas2 = [
        get_char_at_pos(grid, row - 1, col + 1),
        get_char_at_pos(grid, row + 1, col - 1),
    ];

    if (&mas1 == b"MS" || &mas1 == b"SM") && (&mas2 == b"MS" || &mas2 == b"SM") {
        1
    } else {
        0
    }
}

fn process_first(input: String) -> usize {
    let grid = input
        .lines()
        .map(|line| line.as_bytes())
        .collect::<Vec<_>>();

    let mut xmasses = 0;

    for row in 0..grid.len() {
        for col in 0..grid[0].len() {
            // starting from an X
            if grid[row][col] == b'X' {
                xmasses += find_xmas(&grid, row as isize, col as isize);
            }
        }
    }

    xmasses
}

fn process_second(input: String) -> usize {
    let grid = input
        .lines()
        .map(|line| line.as_bytes())
        .collect::<Vec<_>>();

    let mut x_masses = 0;

    for row in 0..grid.len() {
        for col in 0..grid[0].len() {
            // looking around 'A'
            if grid[row][col] == b'A' {
                x_masses += find_mas(&grid, row as isize, col as isize);
            }
        }
    }

    x_masses
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
    const INPUT: &str = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";

    #[test]
    fn test_first() -> Result<()> {
        println!("test1: {:?} ", process_first(INPUT.to_owned()));
        assert_eq!(18, process_first(INPUT.to_owned()));
        Ok(())
    }

    #[test]
    fn test_second() -> Result<()> {
        assert_eq!(9, process_second(INPUT.to_owned()));
        Ok(())
    }
}
