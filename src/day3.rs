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
}
