use std::fs;
use std::io::{Result};

mod day1;
mod day2;

pub fn run() -> Result<()> {
    let input = fs::read_to_string("../res/input-1.txt")?;
    day1::p1(&input);
    day1::p2(&input);

    let input = fs::read_to_string("../res/input-2.txt")?;
    println!("Invalid total: {}", day2::p1(&input));
    println!("Invalid total: {}", day2::p2(&input));

    Ok(())
}
