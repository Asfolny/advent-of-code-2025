use std::io::{self, Result};

fn main() -> Result<()> {
    let mut position = 50;
    let mut zeroes = 0;
    let input = io::stdin().lines();

    for op in input {
        let op = op.unwrap();
        let (dir, mov) = op.split_at(1);
        // The choice of i16 is to handle the 99 + 99 possibility which is too large to fit in i8
        let mov = mov.parse::<i16>().expect("Expected move to be within 255 range, as the dial only goes 0 -> 99") % 100;

        position += match dir {
            "L" => -mov,
            "R" => mov,
            _ => panic!("Unknown operation"),
        };
        position %= 100;

        if position < 0 {
            position += 100;
        }

        if position == 0 {
            zeroes += 1;
        }

    }
    println!("Zero count: {zeroes}");
    Ok(())
}
