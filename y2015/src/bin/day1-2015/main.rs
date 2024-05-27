use std::fs::read_to_string;

fn main() -> anyhow::Result<()> {
    let data = read_to_string("y2015/src/bin/day1-2015/input.txt")?;
    let mut floor = 0;
    let mut basement = 0;
    for (i, c) in data.chars().enumerate() {
        if c == '(' {
            floor += 1;
        } else {
            floor -= 1;
        }

        if floor == -1 && basement == 0 {
            // floors start from 1
            basement = i + 1;
        }
    }

    println!("Part 1: {floor}");
    println!("Part 2: {basement}");
    Ok(())
}
