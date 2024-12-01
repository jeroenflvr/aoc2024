use std::fs;

fn process_first() {
    let input = fs::read_to_string("input1.txt").expect("Error reading file");

    let mut left = vec![];
    let mut right = vec![];

    for line in input.lines() {
        let mut items = line.split_whitespace();
        left.push(items.next().unwrap().parse::<i32>().unwrap());

        right.push(items.next().unwrap().parse::<i32>().unwrap());
    }

    left.sort();
    right.sort();

    let result: i32 = std::iter::zip(left, right)
        .map(|(l, r)| (l - r).abs())
        .sum();

    println!("{result:?}");
}

fn process_second() {
    let input = fs::read_to_string("input1.txt").expect("Error reading file");

    let mut left = vec![];
    let mut right = vec![];

    for line in input.lines() {
        let mut items = line.split_whitespace();
        left.push(items.next().unwrap().parse::<i32>().unwrap());

        right.push(items.next().unwrap().parse::<i32>().unwrap());
    }

    let result: i32 = left
        .iter()
        .map(|n| {
            let count = right.iter().filter(|r| *r == n).count();
            let count_i32: i32 = count.try_into().unwrap();
            n * count_i32
        })
        .sum();

    println!("{result:?}");
}

fn main() {
    process_first();
    process_second()
}
