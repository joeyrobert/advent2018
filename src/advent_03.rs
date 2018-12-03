extern crate regex;
use std::fs::File;
use std::io::Read;
use regex::Regex;

fn coords_to_index(x: i32, y: i32) -> usize {
    return (y * 1000 + x) as usize;
}

fn part_one() {
    let mut f = File::open("input/03.txt").expect("File not found");
    let mut contents = String::new();
    f.read_to_string(&mut contents).expect("File can't be read");
    let split = contents.split("\n");
    let re = Regex::new(r"#(\d+) @ (\d+),(\d+): (\d+)x(\d+)").unwrap();
    let mut values = vec![0; 1000000];

    for s in split {
        if s.len() > 0 {
            let cap = re.captures(s).unwrap();
            let x: i32 = (&cap[2]).parse().unwrap();
            let y: i32 = (&cap[3]).parse().unwrap();
            let width: i32 = (&cap[4]).parse().unwrap();
            let height: i32 = (&cap[5]).parse().unwrap();
            for i in x..(x+width) {
                for j in y..(y+height) {
                    values[coords_to_index(i, j)] += 1;
                }
            }
        }
    }

    let mut total = 0;
    for i in 0..1000000 {
        if values[i] > 1 {
            total += 1;
        }
    }

    println!("{}", total);
}

fn part_two() {
    let mut f = File::open("input/03.txt").expect("File not found");
    let mut contents = String::new();
    f.read_to_string(&mut contents).expect("File can't be read");
    let split: Vec<&str> = contents.split("\n").collect();
    let re = Regex::new(r"#(\d+) @ (\d+),(\d+): (\d+)x(\d+)").unwrap();
    let mut values = vec![0; 1000000];

    for s in &split {
        if s.len() > 0 {
            let cap = re.captures(s).unwrap();
            let x: i32 = (&cap[2]).parse().unwrap();
            let y: i32 = (&cap[3]).parse().unwrap();
            let width: i32 = (&cap[4]).parse().unwrap();
            let height: i32 = (&cap[5]).parse().unwrap();
            for i in x..(x+width) {
                for j in y..(y+height) {
                    values[coords_to_index(i, j)] += 1;
                }
            }
        }
    }

    for s in &split {
        if s.len() > 0 {
            let cap = re.captures(s).unwrap();
            let id: i32 = (&cap[1]).parse().unwrap();
            let x: i32 = (&cap[2]).parse().unwrap();
            let y: i32 = (&cap[3]).parse().unwrap();
            let width: i32 = (&cap[4]).parse().unwrap();
            let height: i32 = (&cap[5]).parse().unwrap();
            let mut is_isolated = true;

            for i in x..(x+width) {
                for j in y..(y+height) {
                    is_isolated = is_isolated && values[coords_to_index(i, j)] == 1;
                }
            }

            if is_isolated {
                println!("{}", id);
                return;
            }
        }
    }
}

fn main() {
    part_one();
    part_two();
}
