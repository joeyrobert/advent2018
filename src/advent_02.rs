use std::fs::File;
use std::io::Read;
use std::collections::HashMap;

fn part_one() {
    let mut f = File::open("input/02.txt").expect("File not found");
    let mut contents = String::new();
    f.read_to_string(&mut contents).expect("File can't be read");
    let split = contents.split("\n");
    let mut two_letters: i32 = 0;
    let mut three_letters: i32 = 0;

    for s in split {
        if s.len() > 0 {
            let mut has_two = false;
            let mut has_three = false;
            let mut frequency = HashMap::new();

            for c in s.split("") {
                if c.len() > 0 {
                    *frequency.entry(c).or_insert(0) += 1;
                }
            }

            for (_k, v) in frequency.iter() {
                if *v == 2 {
                    has_two = true;
                }

                if *v == 3 {
                    has_three = true;
                }
            }

            if has_two {
                two_letters += 1;
            }
            if has_three {
                three_letters += 1;
            }
        }
    }
    println!("{}", two_letters * three_letters);
}

fn letter_difference(a: String, b: String) -> i32 {
    let length = a.trim().len();
    let a_split: Vec<&str> = a.trim().split("").collect();
    let b_split: Vec<&str> = b.trim().split("").collect();
    let mut diff = 0;

    for i in 0..=length {
        if a_split[i] != b_split[i] {
            diff += 1;
        } 
    }

    return diff;
}

fn remove_different_letter(a: String, b: String) -> String {
    let length = a.trim().len();
    let a_split: Vec<&str> = a.trim().split("").collect();
    let b_split: Vec<&str> = b.trim().split("").collect();
    let mut same: String = "".to_string();

    for i in 0..=length {
        if a_split[i] == b_split[i] {
            same += a_split[i];   
        }
    }

    return same;
}

fn part_two() {
    let mut f = File::open("input/02.txt").expect("File not found");
    let mut contents = String::new();
    f.read_to_string(&mut contents).expect("File can't be read");
    let split: Vec<&str> = contents.split("\n").collect();
    let mut min_diff = 10000;
    let mut min_a = "";
    let mut min_b = "";

    for a in &split {
        for b in &split {
            if a.len() > 0 && b.len() > 0 && a != b {
                let diff = letter_difference(a.to_string(), b.to_string());
                if diff < min_diff {
                    min_diff = diff;
                    min_a = a;
                    min_b = b;
                }
            }
        }
    }

    println!("{}", remove_different_letter(min_a.to_string(), min_b.to_string()));
}

fn main() {
    part_one();
    part_two();
}
