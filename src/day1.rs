use std::fs::read_to_string;

pub fn solve() -> u32 {
    let mut result = 0;
    for line in read_to_string("./src/day1_input.txt").unwrap().lines() {
        let mut vals = (0, 0);
        let mut first = false;
        for c in line.chars() {
            if c.is_digit(10) {
                if !first {
                    first = true;
                    vals.0 = c.to_digit(10).unwrap();
                } else {
                    vals.1 = c.to_digit(10).unwrap();
                }
            }
        }
        if vals.1 != 0 {
            result += (vals.0 * 10) + vals.1;
        } else {
            result += (vals.0 * 10) + vals.0;
        }
    }
    result
}
