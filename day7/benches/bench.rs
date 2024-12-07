use std::fs;
use day7::solutions::solutions::{process_second, process_first, process_second_cb, process_second_cb_rayon};

fn main() {
    divan::main();
}


#[divan::bench]
fn bench_p1(){
    let input = fs::read_to_string("input1.txt").expect("Error reading file");

    let second_r = process_first(divan::black_box(input.clone()));
}

#[divan::bench]
fn bench_p2(){
    let input = fs::read_to_string("input1.txt").expect("Error reading file");

    let second_r = process_second(divan::black_box(input.clone()));

}

#[divan::bench]
fn bench_p2_cb_rayon(){
    let input = fs::read_to_string("input1.txt").expect("Error reading file");

    let second_r = process_second_cb_rayon(divan::black_box(input.clone()));

}

#[divan::bench]
fn bench_p2_cb(){
    let input = fs::read_to_string("input1.txt").expect("Error reading file");

    let second_r = process_second_cb(divan::black_box(input.clone()));

}
