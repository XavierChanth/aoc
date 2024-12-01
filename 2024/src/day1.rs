use std::fs::read_to_string;

use itertools::{Either, Itertools};

pub fn day1(q: i32) {
    match q {
        1 => q1(),
        2 => q2(),
        _ => (),
    }
}

fn read_file() -> String {
    let file_path = "src/day1input.txt";
    return read_to_string(file_path).expect("Unable to read {file_path}");
}

fn dist_vecs(a: Vec<i32>, b: Vec<i32>) -> i32 {
    a.into_iter()
        .zip(b)
        .map(|(ae, be)| (ae - be).abs())
        .reduce(|acc, e| acc + e)
        .expect("Dist should succeed")
}

fn q1() {
    // a1 - b1 + a2 - b2 = a1 - b2 + a2 - b1
    // i.e. you don't actually need to sort the list
    let contents = read_file();
    let (mut left, mut right): (Vec<_>, Vec<_>) = contents
        .split_whitespace()
        .map(|e| e.parse::<i32>().expect("Parse should succeed"))
        .enumerate()
        .partition_map(|(i, v)| {
            if i % 2 == 0 {
                Either::Left(v)
            } else {
                Either::Right(v)
            }
        });
    left.sort();
    right.sort();

    let dist = dist_vecs(left, right).to_string();
    println!("List distance: {dist}");
}

fn sim_vecs(a: Vec<i32>, b: Vec<i32>) -> i32 {
    a.iter()
        .map(|x| {
            x * b
                .iter()
                .map(|e| (e == x) as i32)
                .reduce(|acc, e| acc + e)
                .expect("Inner reduce")
        })
        .reduce(|acc, e| acc + e)
        .expect("Outer reduce")
}

fn q2() {
    let contents = read_file();
    let (left, right): (Vec<i32>, Vec<i32>) = contents
        .split_whitespace()
        .map(|e| e.parse::<i32>().expect("Parse should succeed"))
        .enumerate()
        .partition_map(|(i, v)| {
            if i % 2 == 0 {
                Either::Left(v)
            } else {
                Either::Right(v)
            }
        });
    let sim = sim_vecs(left, right).to_string();
    println!("List similarity: {sim}");
}
