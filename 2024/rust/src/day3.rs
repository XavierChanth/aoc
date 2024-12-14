use std::{borrow::Borrow, fs::read_to_string};

pub fn qs(q: i32) {
    match q {
        1 => q1(),
        2 => q2(),
        _ => (),
    }
}

fn read_file() -> String {
    let file_path = "src/day3input.txt";
    return read_to_string(file_path).expect("Unable to read {file_path}");
}

// This is just terrible design don't mind me
// You shouldn't do it this way

enum Status {
    M,
    U,
    L,
    Open,
    A,
    Comma,
    B,
    None,
    D,
    O,
    N,
    Apo, // Apostrophe
    T,
}

fn q1() {
    let mut status: Status = Status::None;
    let mut a = 0;
    let mut depth = 0;
    let mut b = 0;
    let mut sum = 0;
    let contents = read_file();
    contents.chars().for_each(|x| match (status.borrow(), x) {
        (Status::None, 'm') => status = Status::M,
        (Status::M, 'u') => status = Status::U,
        (Status::U, 'l') => status = Status::L,
        (Status::L, '(') => status = Status::Open,
        (Status::A, ',') => {
            depth = 0;
            status = Status::Comma;
        }
        (Status::B, ')') => {
            sum += a * b;
            a = 0;
            b = 0;
            depth = 0;
            status = Status::None;
        }
        (Status::Open, _) => {
            let val = x as i32 - '0' as i32;
            if val < 10 {
                a = val;
                depth += 1;
                status = Status::A;
            } else {
                status = Status::None;
            }
        }
        (Status::A, _) => {
            let val = x as i32 - '0' as i32;
            if val < 10 && depth < 3 {
                a = a * 10 + val;
                depth += 1;
            } else {
                a = 0;
                depth = 0;
                status = Status::None;
            }
        }
        (Status::Comma, _) => {
            let val = x as i32 - '0' as i32;
            if val < 10 {
                b = val;
                depth += 1;
                status = Status::B;
            } else {
                a = 0;
                depth = 0;
                status = Status::None;
            }
        }
        (Status::B, _) => {
            let val = x as i32 - '0' as i32;
            if val < 10 && depth < 3 {
                b = b * 10 + val;
                depth += 1;
            } else {
                a = 0;
                b = 0;
                depth = 0;
                status = Status::None;
            }
        }
        (_, _) => status = Status::None,
    });

    println!("Product Sum: {sum}");
}

enum Op {
    Mul,
    Do,
    Dont,
    None,
}

// Close enough, surely there's a better rust way that I don't know yet
fn q2() {
    let mut status: Status = Status::None;
    let mut a = 0;
    let mut depth = 0;
    let mut b = 0;
    let mut sum = 0;
    let mut op = Op::None;
    let mut dont = false;
    let contents = read_file();
    contents
        .chars()
        .for_each(|x| match (status.borrow(), x, dont) {
            (_, 'd', _) => {
                a = 0;
                b = 0;
                depth = 0;
                status = Status::D;
            }
            (Status::D, 'o', _) => status = Status::O,
            (Status::O, 'n', _) => status = Status::N,
            (Status::N, '\'', _) => status = Status::Apo,
            (Status::Apo, 't', _) => status = Status::T,
            (Status::O, '(', _) => {
                op = Op::Do;
                status = Status::N
            }
            (Status::T, '(', _) => {
                op = Op::Dont;
                status = Status::Open;
            }
            (Status::Open, ')', _) => {
                match op {
                    Op::Dont => dont = true,
                    Op::Do => dont = false,
                    _ => {}
                };
                op = Op::None;
                status = Status::None
            }
            (_, _, true) => {
                // Don't is enabled
                op = Op::None;
                status = Status::None
            }
            (Status::None, 'm', _) => status = Status::M,
            (Status::M, 'u', _) => status = Status::U,
            (Status::U, 'l', _) => status = Status::L,
            (Status::L, '(', _) => {
                op = Op::Mul;
                status = Status::Open;
            }

            (Status::A, ',', _) => {
                depth = 0;
                status = Status::Comma;
            }
            (Status::B, ')', _) => {
                sum += a * b;
                a = 0;
                b = 0;
                depth = 0;
                status = Status::None;
            }
            (Status::Open, _, _) => {
                let val = x as i32 - '0' as i32;
                if val < 10 {
                    a = val;
                    depth += 1;
                    status = Status::A;
                } else {
                    status = Status::None;
                }
            }
            (Status::A, _, _) => {
                let val = x as i32 - '0' as i32;
                if val < 10 && depth < 3 {
                    a = a * 10 + val;
                    depth += 1;
                } else {
                    a = 0;
                    depth = 0;
                    status = Status::None;
                }
            }
            (Status::Comma, _, _) => {
                let val = x as i32 - '0' as i32;
                if val < 10 {
                    b = val;
                    depth += 1;
                    status = Status::B;
                } else {
                    a = 0;
                    depth = 0;
                    status = Status::None;
                }
            }
            (Status::B, _, _) => {
                let val = x as i32 - '0' as i32;
                if val < 10 && depth < 3 {
                    b = b * 10 + val;
                    depth += 1;
                } else {
                    a = 0;
                    b = 0;
                    depth = 0;
                    status = Status::None;
                }
            }
            (_, _, _) => status = Status::None,
        });

    println!("Product Sum: {sum}");
}
