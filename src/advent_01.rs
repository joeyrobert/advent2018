use std::fs::File;
use std::io::Read;
use std::collections::HashMap;

fn part_one() {
    let mut f = File::open("input/01.txt").expect("File not found");
    let mut contents = String::new();
    f.read_to_string(&mut contents).expect("File can't be read");
    let split = contents.split("\n");
    let mut total: i32 = 0;
    for s in split {
        if s.len() > 0 {
            let delta: i32 = s.parse().unwrap();
            total += delta;
        }
    }
    println!("{}", total);
}

fn part_two() {
    let mut f = File::open("input/01.txt").expect("File not found");
    let mut contents = String::new();
    f.read_to_string(&mut contents).expect("File can't be read");
    let split: Vec<&str> = contents.split("\n").collect();
    let mut frequencies = HashMap::new();
    let mut total: i32 = 0;
    frequencies.insert(0, 1);

    loop {
        for s in &split {
            if s.len() > 0 {
                let delta: i32 = s.parse().unwrap();
                total += delta;
                *frequencies.entry(total).or_insert(0) += 1;
                if frequencies.get(&total).unwrap() == &2 {
                    println!("{}", total);
                    return;
                }
            }
        }
    }
}

fn main() {
    part_one();
    part_two();
}
