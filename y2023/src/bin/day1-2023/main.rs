use std::fs::*;
use std::str::from_utf8;

fn set_val(vals: &mut (u32, u32), first: &mut bool, num: u32) {
    if !(*first) {
        *first = true;
        vals.0 = num;
    } else {
        vals.1 = num;
    }
}

fn extract_slice(bytes: &[u8], curr: usize, offset: usize) -> Option<&str> {
    if curr + offset < bytes.len() {
        let slice = &bytes[curr + 1..curr + offset + 1];
        let str_slice = from_utf8(&slice).unwrap();
        Some(str_slice)
    } else {
        None
    }
}

fn main() {
    let mut result = 0;
    for line in read_to_string("y2023/src/bin/day1-2023/input.txt")
        .unwrap()
        .lines()
    {
        let mut vals = (0, 0);
        let mut first = false;
        let bytes = line.as_bytes();
        let mut index = 0;
        loop {
            if index >= bytes.len() {
                break;
            }
            let c = bytes[index] as char;
            match c {
                '0' => {
                    set_val(&mut vals, &mut first, 0);
                    index += 1;
                }
                '1' => {
                    set_val(&mut vals, &mut first, 1);
                    index += 1;
                }
                '2' => {
                    set_val(&mut vals, &mut first, 2);
                    index += 1;
                }
                '3' => {
                    set_val(&mut vals, &mut first, 3);
                    index += 1;
                }
                '4' => {
                    set_val(&mut vals, &mut first, 4);
                    index += 1;
                }
                '5' => {
                    set_val(&mut vals, &mut first, 5);
                    index += 1;
                }
                '6' => {
                    set_val(&mut vals, &mut first, 6);
                    index += 1;
                }
                '7' => {
                    set_val(&mut vals, &mut first, 7);
                    index += 1;
                }
                '8' => {
                    set_val(&mut vals, &mut first, 8);
                    index += 1;
                }
                '9' => {
                    set_val(&mut vals, &mut first, 9);
                    index += 1;
                }
                'o' => {
                    if extract_slice(&bytes, index, 2) == Some("ne") {
                        set_val(&mut vals, &mut first, 1);
                        index += 2;
                    } else {
                        index += 1;
                    }
                }
                't' => {
                    if extract_slice(&bytes, index, 2) == Some("wo") {
                        set_val(&mut vals, &mut first, 2);
                        index += 2;
                    } else if extract_slice(&bytes, index, 4) == Some("hree") {
                        set_val(&mut vals, &mut first, 3);
                        index += 4;
                    } else {
                        index += 1;
                    }
                }
                'f' => {
                    if extract_slice(&bytes, index, 3) == Some("our") {
                        set_val(&mut vals, &mut first, 4);
                        index += 3;
                    } else if extract_slice(&bytes, index, 3) == Some("ive") {
                        set_val(&mut vals, &mut first, 5);
                        index += 3;
                    } else {
                        index += 1;
                    }
                }
                's' => {
                    if extract_slice(&bytes, index, 2) == Some("ix") {
                        set_val(&mut vals, &mut first, 6);
                        index += 2;
                    } else if extract_slice(&bytes, index, 4) == Some("even") {
                        set_val(&mut vals, &mut first, 7);
                        index += 4;
                    } else {
                        index += 1;
                    }
                }
                'e' => {
                    if extract_slice(&bytes, index, 4) == Some("ight") {
                        set_val(&mut vals, &mut first, 8);
                        index += 4;
                    } else {
                        index += 1;
                    }
                }
                'n' => {
                    if extract_slice(&bytes, index, 3) == Some("ine") {
                        set_val(&mut vals, &mut first, 9);
                        index += 3;
                    } else {
                        index += 1;
                    }
                }
                _ => index += 1,
            }
        }
        let intermediate = if vals.1 != 0 {
            (vals.0 * 10) + vals.1
        } else {
            (vals.0 * 10) + vals.0
        };
        result += intermediate;
    }
    println!("{}", result);
}
