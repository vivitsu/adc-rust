use std::fs::*;

pub fn solve() -> u32 {
    let mut result = 0;
    for line in read_to_string("./src/day1_input.txt").unwrap().lines() {
        let mut vals = (0, 0);
        let mut first = false;
        let bytes = line.as_bytes();
        let mut index = 0;
        loop {
            if index >= bytes.len() {
                break;
            }
            let c = bytes[index] as char;
            if c.is_digit(10) {
                if !first {
                    first = true;
                    vals.0 = c.to_digit(10).unwrap();
                } else {
                    vals.1 = c.to_digit(10).unwrap();
                }
                index += 1;
            } else {
                match c {
                    'o' => {
                        // should be better with slices i think
                        if bytes[index + 1] as char == 'n' && bytes[index + 2] as char == 'e' {
                            if !first {
                                first = true;
                                vals.0 = 1;
                            } else {
                                vals.1 = 1;
                            }
                            index += 3;
                        }
                    }
                    _ => index += 1,
                }
            }
        }
        let intermediate = if vals.1 != 0 {
            (vals.0 * 10) + vals.1
        } else {
            (vals.0 * 10) + vals.0
        };
        result += intermediate;
    }
    println!("result: {}", result);
    result
}
