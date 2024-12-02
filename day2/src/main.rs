use anyhow::Result;
use std::fs;

#[derive(Debug, PartialEq)]
enum Direction {
    Up,
    Down,
}

fn check_row(items: Vec<i32>) -> Option<usize> {
    let direction: Direction = match items[0] > items[1] {
        true => Direction::Down,
        false => Direction::Up,
    };

    for i in 0..items.len() - 1 {
        let n1 = items[i + 1];
        let n0 = items[i];

        if direction == Direction::Up {
            if n1 - n0 <= 0 || n1 - n0 > 3 {
                return Some(i);
            }
        } else {
            if n1 - n0 >= 0 || n0 - n1 > 3 {
                return Some(i);
            }
        }
    }
    None
}

fn process_first(input: String) -> u16 {
    let mut safes: u16 = 0;
    let mut unsafes: u16 = 0;

    for line in input.lines() {
        let items = line
            .split_whitespace()
            .map(|x| x.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();
        let is_safe = check_row(items.clone());

        match is_safe {
            None => safes += 1,
            Some(_) => unsafes += 1,
        };
    }

    safes
}

fn problem_dampener(items: Vec<i32>, mut without_index: usize) -> Option<usize> {
    let mut new_items0 = items.clone();
    new_items0.remove(without_index);
    let test_0 = check_row(new_items0.clone());

    match test_0 {
        None => None,
        Some(i) => {
            let mut new_items1 = items.clone();
            new_items1.remove(without_index + 1);

            if let Some(_) = check_row(new_items1.clone()) {
                // direction changed, check if it changed earlier
                if without_index > 0 {
                    if let Some(_) = problem_dampener(items.clone(), without_index - 1) {
                        Some(i)
                    } else {
                        None
                    }
                } else {
                    Some(i)
                }
            } else {
                None
            }
        }
    }
}

fn process_second(input: String) -> u16 {
    let mut safes: u16 = 0;
    let mut unsafes: u16 = 0;

    for line in input.lines() {
        let items = line
            .split_whitespace()
            .map(|x| x.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();
        let check_safety = check_row(items.clone());

        match check_safety {
            None => {
                safes += 1;
            }
            Some(ind) => {
                if let Some(_) = problem_dampener(items.clone(), ind) {
                    unsafes += 1;
                } else {
                    safes += 1;
                };
            }
        }
    }

    safes
}

fn main() {
    let input = fs::read_to_string("input1.txt").expect("Error reading file");

    let first_r = process_first(input.clone());
    let second_r = process_second(input.clone());

    println!("{first_r}");
    println!("{second_r}");
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
    fn test_first() -> Result<()> {
        assert_eq!(2, process_first(INPUT.to_owned()));
        Ok(())
    }

    #[test]
    fn test_second() -> Result<()> {
        assert_eq!(4, process_second(INPUT.to_owned()));
        Ok(())
    }
}
