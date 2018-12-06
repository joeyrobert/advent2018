extern crate regex;
use std::fs::File;
use std::io::Read;

fn react(value: String) -> String {
    let chars: Vec<&str> = value.split("").collect();
    let mut total_str: Vec<&str> = vec![];
    let mut i = 0;

    while i < chars.len() - 1 {
        let a = chars[i];
        let b = chars[i + 1];
        if *a != *b && ((*a).to_uppercase() == (*b) || (*b).to_uppercase() == (*a)) {
            i += 2;
        } else {
            total_str.push(a);
            i += 1;
        }
    }

    total_str.push(chars[chars.len() - 1]);

    return total_str.join("");
}

fn part_one() {
    let mut f = File::open("input/05.txt").expect("File not found");
    let mut contents = String::new();
    f.read_to_string(&mut contents).expect("File can't be read");

    let mut before_length = contents.len();
    while {
        contents = react(contents);
        let after_length = contents.len();
        let condition = before_length != after_length;
        before_length = after_length;
        condition
    } { }

    println!("{}", contents.len());
}

fn part_two() {
    let mut f = File::open("input/05.txt").expect("File not found");
    let mut contents = String::new();
    f.read_to_string(&mut contents).expect("File can't be read");
    let contents_copy = contents.clone();
    let mut chars: Vec<&str> = contents_copy.split("").collect();
    chars.sort();
    chars.dedup();
    let mut min_length = contents.len();

    for ch in chars {
        let mut local_contents = contents.clone().replace(&(*ch).to_string(), "").replace(&(*ch).to_lowercase(), "");
        let mut before_length = contents.len();
        while {
            local_contents = react(local_contents);
            let after_length = local_contents.len();
            let condition = before_length != after_length;
            before_length = after_length;
            condition
        } { }

        if local_contents.len() < min_length {
            min_length = local_contents.len();
        }
    }

    println!("{}", min_length);
}

fn main() {
    part_one();
    part_two();
}
