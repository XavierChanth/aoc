use std::fs::read_to_string;

use iterslide::SlideIterator;

pub fn qs(q: i32) {
    match q {
        1 => q1(),
        2 => q2(),
        _ => (),
    }
}

fn read_file() -> String {
    let file_path = "src/day2input.txt";
    return read_to_string(file_path).expect("Unable to read {file_path}");
}

fn line_to_vec(s: &str) -> Vec<i32> {
    s.split_whitespace()
        .map(|x| x.parse::<i32>().expect("Should be an integer"))
        .collect()
}

fn is_safe<I, J>(levels: I) -> i32
where
    I: SlideIterator<J, J::Item>,
    J: Iterator<Item = i32>,
{
    let mut v = 0;
    for w in levels.slide(2) {
        let diff = w[0] - w[1];

        if diff == 0 {
            return 0;
        }
        let norm = diff.abs();
        let direction = v * diff;
        if direction == 0 {
            v = diff / norm;
        }
        if (v * diff < 0) || (norm < 1) || (norm > 3) {
            return 0;
        }
    }
    1
}

fn q1() {
    let contents = read_file();
    let count = contents
        .lines()
        .map(line_to_vec)
        .map(is_safe)
        .reduce(|acc, s| acc + s)
        .expect("Sum succeeds");

    println!("Safe count: {count}");
}

// This is not efficient at all, but it's fine.
// I'm still learning rust, I'm trying to stick
// to the rust way as much as possible, but I'm
// not comfortable enough with iterators yet
// I don't know how to do sliding windows with
// iterators, which would make this much easier
// to optimize
fn is_safe2(levels: Vec<i32>) -> i32 {
    if is_safe(levels.to_vec()) == 1 {
        return 1;
    } else {
        for i in 0..levels.len() {
            let mut skipped = levels.to_vec();
            skipped.remove(i);
            if is_safe(skipped) == 1 {
                return 1;
            }
        }
    }
    0
}

// Nah dude... I'm annoyed... I kept getting the same answer...
// It wasn't until the third time that it accepted that answer as correct
fn q2() {
    let contents = read_file();
    let count = contents
        .lines()
        .map(line_to_vec)
        .map(is_safe2)
        .reduce(|acc, s| acc + s)
        .expect("Sum succeeds");

    println!("Safe count: {count}");
}
