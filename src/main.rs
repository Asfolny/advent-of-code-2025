use std::io::{self, Result};
use std::cmp;

fn main() -> Result<()> {
    let input: Vec<String> = io::stdin().lines().map(|line| line.unwrap()).collect();
    p1(&input)?;
    p2(&input)?;
    Ok(())
}

fn p1(input: &Vec<String>) -> Result<()> {
    let mut position = 50;
    let mut zeroes = 0;

    for op in input {
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

fn p2(input: &Vec<String>) -> Result<()> {
    let mut position = 50;
    let mut zeroes = 0;

    for op in input {
        let (dir, mov) = op.split_at(1);
        let mov = mov.parse::<i32>().expect("Number too large or NaN");
        let start = position;
        position += match dir {
            "L" => -mov,
            "R" => mov,
            _ => panic!("Unknown operation"),
        };

        if position > 0 && position < 100 {
            continue;
        }

        if dir == "R" {
            zeroes += position / 100;
            position %= 100;
        } else {
            let mut rots = cmp::max(position.abs() / 100, 1);
            if position <= -100 {
                rots += 1;
            }
            position %= 100;

            if position < 0 {
                position += 100;
            }

            if start == 0 {
                rots -= 1;
            }
            zeroes += rots;
        }
    }
    println!("Zero count: {zeroes}");
    Ok(()) 
}
