use std::fs;

fn check_compute(
    v_result: u128,
    terms: &Vec<&str>,
    i: usize,
    current_result: u128,
    test_concat: bool,
) -> bool {
    if i == terms.len() {
        return v_result == current_result;
    }

    if check_compute(
        v_result,
        terms,
        i + 1,
        current_result + terms[i].parse::<u128>().unwrap(),
        test_concat,
    ) {
        return true;
    }
    if check_compute(
        v_result,
        terms,
        i + 1,
        current_result * terms[i].parse::<u128>().unwrap(),
        test_concat,
    ) {
        return true;
    }

    // test concatenating for the || operator
    if test_concat {
        if check_compute(
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

fn process_first(input: String) -> u128 {
    let mut good_results = vec![];

    for line in input.split("\n") {
        let (v, t) = line.split_once(":").unwrap();
        let validate_result = v.parse::<u128>().unwrap();
        let terms = t.split_whitespace().collect();

        println!("{validate_result}:  {:?}", terms);
        if check_compute(validate_result, &terms, 0, 0, false) {
            good_results.push(validate_result)
        }
    }

    good_results.iter().sum()
}

fn process_second(input: String) -> u128 {
    let mut good_results = vec![];

    for line in input.split("\n") {
        let (v, t) = line.split_once(":").unwrap();
        let validate_result = v.parse::<u128>().unwrap();
        let terms = t.split_whitespace().collect();

        println!("{validate_result}:  {:?}", terms);
        if check_compute(validate_result, &terms, 0, 0, true) {
            good_results.push(validate_result)
        }
    }

    good_results.iter().sum()
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
}
