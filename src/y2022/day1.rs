use std::fs::read_to_string;

pub fn solve() -> u32 {
    let contents = read_to_string("src/y2022/day1.txt").expect("Unable to read input file!");
    let mut split: Vec<&str> = contents.as_str().rsplit('\n').collect();
    split.reverse();

    let mut last = 0;
    let mut nelves = 0;
    let mut elf_calories: Vec<(u32, u32)> = Vec::new();

    for index in 0..split.len() {
        if split[index] == "" {
            nelves += 1;
            let mut total_calories = 0;
            for index in last..index {
                let val: u32 = split[index].parse().unwrap();
                total_calories += val;
            }
            elf_calories.push((nelves, total_calories));
            last = index + 1;
        }
    }

    elf_calories.sort_by(|a, b| a.1.cmp(&b.1));
    elf_calories.reverse();

    let mut acc = 0;
    for index in 0..3 {
        acc += elf_calories[index].1;
    }
    
    return acc;
}


