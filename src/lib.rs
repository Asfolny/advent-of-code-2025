use std::fs::File;
use std::io::{Result, Seek, SeekFrom};

mod day1;

pub fn run() -> Result<()> {
    let mut input = File::open("../res/input-1.txt")?;
    day1::p1(&input)?;
    input.seek(SeekFrom::Start(0))?;
    day1::p2(&input)?;

    Ok(())
}
