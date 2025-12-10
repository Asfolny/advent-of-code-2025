pub fn p1(input: &str) -> u64 {
    let mut result = 0;
    let mut ranges = Vec::new();
    let mut lines = input.trim().split('\n').collect::<Vec<_>>();
    let split_idx = lines.iter().position(|l| l.is_empty()).expect("Input does not contain blank line dividing ranges and fruits");
    let range_lines = lines.drain(..split_idx).collect::<Vec<_>>();
    let fruits = lines.iter().skip(1).map(|n| n.parse::<u64>().expect("Invalid u64 fruit number")).collect::<Vec<_>>();

    for range_str in range_lines {
        let range_parts = range_str
            .split('-')
            .map(|n| n.parse::<u64>().expect("Invalid u64 number in range"))
            .collect::<Vec<u64>>();

        if range_parts.len() != 2 {
            panic!("Invalid range provided");
        }

        let (l, r) = (range_parts[0], range_parts[1]);

        ranges.push(l..=r);
    }

    'fruit_loop: for fruit in fruits {
        for range in &ranges {
            if range.contains(&fruit) {
                result += 1;
                continue 'fruit_loop
            }
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_p1() {
        let input = "
3-5
10-14
16-20
12-18

1
5
8
11
17
32
";
        let result = p1(input);
        assert_eq!(result, 3);
    }
}
