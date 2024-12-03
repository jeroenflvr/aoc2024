use regex::Regex;
use std::fs;

fn mul(x: u32, y: u32) -> u32 {
    x * y
}

fn eval(input: &str) -> u32 {
    let re = Regex::new(r"\d{1,3}").unwrap();
    let mut nums: Vec<u32> = Vec::new();

    for mat in re.find_iter(input) {
        nums.push(mat.as_str().parse::<u32>().unwrap());
    }
    mul(nums[0], nums[1])
}

fn process_first(input: String) -> u32 {
    let mut results: u32 = 0;
    let re = Regex::new(r"mul\(\d{1,3}\,\d{1,3}\)").unwrap();

    for mat in re.find_iter(&input) {
        let r = eval(mat.as_str());
        results += r;
    }
    results
}

fn process_second(input: String) -> u32 {
    let mut results: u32 = 0;

    let re_target = Regex::new(r"mul\(\d{1,3},\d{1,3}\)|don't\(\)|do\(\)").unwrap();
    let mut enabled = true;

    for mat in re_target.find_iter(&input) {
        let m = mat.as_str();
        let enabled_n = match m {
            "don't()" => false,
            "do()" => true,
            _ => enabled,
        };

        if enabled_n && m.starts_with("mul") {
            results += eval(m)
        }
        enabled = enabled_n
    }
    results
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
    const INPUT: &str = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
    const INPUT2: &str =
        "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";

    #[test]
    fn test_first() -> Result<()> {
        assert_eq!(161, process_first(INPUT.to_owned()));
        Ok(())
    }

    #[test]
    fn test_second() -> Result<()> {
        assert_eq!(48, process_second(INPUT2.to_owned()));
        Ok(())
    }
}
