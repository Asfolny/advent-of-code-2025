use std::fs;
use std::io::{Result};

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;

pub fn run() -> Result<()> {
    // let input = fs::read_to_string("./res/input-1.txt")?;
    // day1::p1(&input);
    // day1::p2(&input);
   
    // let input = fs::read_to_string("./res/input-2.txt")?;
    // println!("Invalid total: {}", day2::p1(&input));
    // println!("Invalid total: {}", day2::p2(&input));

    // let input = fs::read_to_string("./res/input-3.txt")?;
    // println!("Max joltage: {}", day3::p1(&input));
    // println!("Max joltage: {}", day3::p2(&input));

    // let input = fs::read_to_string("./res/input-4.txt")?;
    // println!("Movable boxes: {}", day4::p1(&input));
    
    let input = fs::read_to_string("./res/input-5.txt")?;
    println!("Fresh fruit: {}", day5::p1(&input));

    Ok(())
}
