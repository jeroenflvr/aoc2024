use itertools::Itertools;
use rayon::prelude::*;
use std::fs;

fn check_compute_dfs(
    v_result: u64,
    terms: &Vec<&str>,
    i: usize,
    current_result: u64,
    test_concat: bool,
) -> bool {
    if i == terms.len() {
        return v_result == current_result;
    }

    if check_compute_dfs(
        v_result,
        terms,
        i + 1,
        current_result + terms[i].parse::<u64>().unwrap(),
        test_concat,
    ) {
        return true;
    }

    if check_compute_dfs(
        v_result,
        terms,
        i + 1,
        current_result * terms[i].parse::<u64>().unwrap(),
        test_concat,
    ) {
        return true;
    }

    // test concatenating for the || operator
    if test_concat {
        if check_compute_dfs(
            v_result,
            terms,
            i + 1,
            (current_result.to_string() + terms[i]).parse().unwrap(),
            test_concat,
        ) {
            return true;
        }
    }
    false
}

pub fn process_first(input: String) -> u64 {
    let mut good_results = vec![];

    for line in input.split("\n") {
        let (v, t) = line.split_once(":").unwrap();
        let validate_result = v.parse::<u64>().unwrap();
        let terms = t.split_whitespace().collect();

        if check_compute_dfs(validate_result, &terms, 0, 0, false) {
            good_results.push(validate_result)
        }
    }

    good_results.iter().sum()
}

pub fn process_second(input: String) -> u64 {
    let mut good_results = vec![];

    for line in input.split("\n") {
        let (v, t) = line.split_once(":").unwrap();
        let validate_result = v.parse::<u64>().unwrap();
        let terms = t.split_whitespace().collect();

        if check_compute_dfs(validate_result, &terms, 0, 0, true) {
            good_results.push(validate_result)
        }
    }

    good_results.iter().sum()
}


// Chris Biscardi's solution: https://youtu.be/iNmHv2Gevps?si=lp6rzrlBIw2AjcY8
pub fn process_second_cb(input: String) -> u64 {
    let ops: [char; 3] = ['*', '+', '_']; // _ is unused, just says we need to do something else
                                          // let mut good_results = vec![];
    let mut test_lines = vec![];

    for line in input.lines() {
        test_lines.push(
            line.split_once(":")
                .map(|(x, term)| {
                    (
                        x.parse::<u64>().unwrap(),
                        term.split_ascii_whitespace()
                            .map(|x| x.parse::<u64>().unwrap())
                            .collect::<Vec<u64>>(),
                    )
                })
                .unwrap(),
        )
    }

    test_lines
        .iter()
        .filter_map(|(test, numbers)| {
            let op_count = numbers.len() - 1;
            (0..op_count)
                .map(|_| ops)
                .multi_cartesian_product()
                .any(|seq| {
                    let mut sequence = seq.iter();
                    let result = numbers
                        .iter()
                        .copied()
                        .reduce(|acc, next_number| match sequence.next().unwrap() {
                            '*' => acc * next_number,
                            '+' => acc + next_number,
                            '_' => format!("{}{}", acc, next_number).parse::<u64>().unwrap(),
                            _ => panic!("invalid operator"),
                        })
                        .unwrap();
                    *test == result
                })
                .then_some(test)
        })
        .sum()

    // good_results.iter().sum()
}



pub fn process_second_cb_rayon(input: String) -> u64 {
    let ops: [char; 3] = ['*', '+', '_']; // _ is unused, just says we need to do something else
    // let mut good_results = vec![];
    let mut test_lines = vec![];

    for line in input.lines() {
        test_lines.push(
            line.split_once(":")
                .map(|(x, term)| {
                    (
                        x.parse::<u64>().unwrap(),
                        term.split_ascii_whitespace()
                            .map(|x| x.parse::<u64>().unwrap())
                            .collect::<Vec<u64>>(),
                    )
                })
                .unwrap(),
        )
    }

    test_lines
        .par_iter()
        .filter_map(|(test, numbers)| {
            let op_count = numbers.len() - 1;
            (0..op_count)
                .map(|_| ops)
                .multi_cartesian_product()
                .any(|seq| {
                    let mut sequence = seq.iter();
                    let result = numbers
                        .iter()
                        .copied()
                        .reduce(|acc, next_number| match sequence.next().unwrap() {
                            '*' => acc * next_number,
                            '+' => acc + next_number,
                            '_' => format!("{}{}", acc, next_number).parse::<u64>().unwrap(),
                            _ => panic!("invalid operator"),
                        })
                        .unwrap();
                    *test == result
                })
                .then_some(test)
        })
        .sum()

    // good_results.iter().sum()
}

// fn main() {
//     let input = fs::read_to_string("input1.txt").expect("Error reading file");

//     let first_r = process_first(input.clone());
//     let second_r = process_second(input.clone());
//     let second_r_cb = process_second_cb(input.clone());
//     let second_r_cb_rayon = process_second_cb_rayon(input.clone());

//     println!("first: {first_r}");
//     println!("second: {second_r}");
//     println!("second cb: {second_r_cb}");
//     println!("second cb rayon: {second_r_cb_rayon}");
// }

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;
    const INPUT: &str = "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";

    #[test]
    fn test_first() -> Result<()> {
        assert_eq!(3749, process_first(INPUT.to_owned()));
        Ok(())
    }

    #[test]
    fn test_second() -> Result<()> {
        assert_eq!(11387, process_second(INPUT.to_owned()));
        Ok(())
    }

    #[test]
    fn test_second_cb() -> Result<()> {
        assert_eq!(11387, process_second_cb(INPUT.to_owned()));
        Ok(())
    }
    #[test]
    fn test_second_cb_rayon() -> Result<()> {
        assert_eq!(11387, process_second_cb_rayon(INPUT.to_owned()));
        Ok(())
    }
}

// #[divan::bench]
// fn bench_p2(){
//     let input = fs::read_to_string("input1.txt").expect("Error reading file");
//
//     let second_r = process_second(divan::black_box(input.clone()));
//     println!("second_r: {second_r}");
//
// }
