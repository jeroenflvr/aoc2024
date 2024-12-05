use std::collections::HashSet;
use std::fs;

fn process_first(input: String) -> usize {
    let (rules_lines, update_lines) = input.split_once("\n\n").unwrap();

    let mut rules = HashSet::new();

    for line in rules_lines.lines(){
        let (page1, page2) = line.split_once("|").unwrap();
        let p1 = page1.parse::<usize>().unwrap();
        let p2 = page2.parse::<usize>().unwrap();

        rules.insert((p1, p2));
    }

    let result = update_lines.lines()
        .map(|update| {
            update
                .split(",")
                .filter_map(|x| {
                    let trimmed = x.trim();
                    if trimmed.is_empty() {
                        None
                    } else {
                        trimmed.parse::<usize>().ok()
                    }
                })
                .collect::<Vec<_>>()
        })
            .filter(|update| {
                !update.is_empty() && update.is_sorted_by(|before, after| !rules.contains(&(*after, *before)))
            })
            .map(|update| update[update.len() / 2])
            .sum();

    result
}

fn process_second(input: String) -> usize {
    let (rules_lines, update_lines) = input.split_once("\n\n").unwrap();

    let mut rules = HashSet::new();

    for line in rules_lines.lines(){
        let (page1, page2) = line.split_once("|").unwrap();
        let p1 = page1.parse::<usize>().unwrap();
        let p2 = page2.parse::<usize>().unwrap();

        rules.insert((p1, p2));
    }

    let result = update_lines.lines()
        .map(|update| {
            update
                .split(",")
                .filter_map(|x| {
                    let trimmed = x.trim();
                    if trimmed.is_empty() {
                        None
                    } else {
                        trimmed.parse::<usize>().ok()
                    }
                })
                .collect::<Vec<_>>()
        })
        .filter(|update| {
            !update.is_sorted_by(|before, after| !rules.contains(&(*after, *before)))
        })
        .map(|mut update| {
                 update.sort_by(|before,after| rules.contains(&(*before,*after)).cmp(&true));
                update[update.len()/2]})
        .sum();


    result
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
    const INPUT: &str = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";

    #[test]
    fn test_first() -> Result<()> {
        assert_eq!(143, process_first(INPUT.to_owned()));
        Ok(())
    }

    #[test]
    fn test_second() -> Result<()> {
        assert_eq!(123, process_second(INPUT.to_owned()));
        Ok(())
    }
}
