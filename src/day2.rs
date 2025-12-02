pub fn p1(input: &str) -> u64 {
    let mut result: u64 = 0;

    for range in input.split(',') {
        let [first, last]: [u64; 2] = range
            .split('-')
            .take(2)
            .map(|p| {
                p.trim()
                    .parse::<u64>()
                    .expect("Each side of the range must be a valid u64 number")
            })
            .collect::<Vec<_>>()
            .try_into()
            .expect("Could not find left and right sides of <l>-<r> format");

        for p in first..=last {
            let pstr = p.to_string();
            if pstr.len() % 2 == 1 {
                continue;
            } // odd number, always valid
            let (high, low) = pstr.split_at(pstr.len() / 2);
            if low == high {
                result += p;
            }
        }
    }

    result
}

pub fn p2(input: &str) -> u64 {
    let mut result: u64 = 0;

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_p1() {
        let input = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";
        assert_eq!(p1(input), 1227775554)
    }

    #[test]
    fn sample_p2() {
        let input = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";
        assert_eq!(p2(input), 4174379265)
    }
}
