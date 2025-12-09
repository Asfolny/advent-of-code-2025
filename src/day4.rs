use std::cmp;

pub fn p1(input: &str) -> u64 {
    let mut result = 0;
    let input = input.trim();
    let dirs = vec![(-1, -1), (-1, 0), (-1, 1), (0, -1), (0, 1), (1, -1), (1, 0), (1, 1)];
    let rows = input.split('\n');
    let row_count = rows.clone().count();

    for (r, row) in rows.enumerate() {
        let col_count = row.chars().count();

        for (c, point) in row.chars().enumerate() {
            if point != '@' {
                continue;
            }

            let mut surround = 0;

            for dir in &dirs {
                let r = r as i32 + dir.0;
                let c = c as i32 + dir.1;

                if cmp::min(r, c) < 0
                || r >= row_count as i32 
                || c >= col_count as i32
                || input.split('\n')
                    .skip(r as usize)
                    .next()
                    .expect("Available row")
                    .chars()
                    .skip(c as usize)
                    .next()
                    .expect("Available col") != '@' {
                    continue;
                }

                surround += 1;
            }

            if surround < 4 {
                result += 1;
            }
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = "
..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.
";
        assert_eq!(p1(input), 13);
    }
}
