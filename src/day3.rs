pub fn p1(input: &str) -> u32 {
    let mut result = 0;

    for bank in input.trim().lines() {
        let mut high_idx = 0;
        let mut high = bank.chars().nth(high_idx).unwrap();

        for idx in 1..bank.len()-1 {
            let num = bank.chars().nth(idx).unwrap();
            if num > high {
                high_idx = idx;
                high = num;
            }
        }

        let mut low = bank.chars().nth(high_idx + 1).unwrap();
        for num in bank.chars().skip(high_idx + 2) {
            if num > low {
                low = num;
            }
        }

        result += format!("{high}{low}").parse::<u32>().unwrap();
    }

    result
}

pub fn p2(input: &str) -> u64 {
    let mut result: u64 = 0;

    for bank in input.trim().lines() {
        let mut bank_line = Vec::new();
        let mut left_idx = 0;
        
        for run in 1..=12 {
            let mut highest_so_far = bank.chars().nth(left_idx).unwrap(); 
            left_idx += 1;

            for idx in left_idx..bank.len()-12+run {
                let num = bank.chars().nth(idx).unwrap();
                
                if num > highest_so_far {
                    highest_so_far = num;
                    left_idx = idx+1;
                }
            }

            bank_line.push(highest_so_far);
        }

        let bank_output = bank_line.into_iter().collect::<String>();
        result += bank_output.parse::<u64>().unwrap();
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_p1() {
        let input = "
987654321111111
811111111111119
234234234234278
818181911112111
";
        assert_eq!(p1(input), 357);
    }

    #[test]
    fn sample_p2() {
        let input = "
987654321111111
811111111111119
234234234234278
818181911112111
";
        assert_eq!(p2(input), 3121910778619 as u64);
    }
}
