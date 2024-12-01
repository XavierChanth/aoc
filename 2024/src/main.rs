mod day1;

use crate::day1::day1;

use std::{env::args, process::exit};

fn main() {
    let mut a = args();
    a.next(); // drop argv[0]
    let iday = a.next();
    if iday == None {
        println!("Day not provided");
        exit(1);
    }
    let iq = a.next();
    if iq == None {
        println!("Question not provided");
        exit(1);
    }
    let day = iday.unwrap().parse::<i32>().unwrap();
    let q = iq.unwrap().parse::<i32>().unwrap();
    match day {
        1 => day1(q),
        _ => (),
    }
}
