extern crate regex;
extern crate skiplist;
use std::fs::File;
use std::io::Read;
use std::collections::HashMap;
use regex::Regex;

fn get_data() -> (Vec<i64>, Vec<i64>, Vec<i64>, Vec<i64>) {
    let mut f = File::open("input/10.txt").expect("File not found");
    let mut contents = String::new();
    f.read_to_string(&mut contents).expect("File can't be read");
    let split = contents.split("\n");
    let re = Regex::new(r"^position=<(.*?),(.*?)> velocity=<(.*?),(.*?)>$").unwrap();
    let mut position_x: Vec<i64> = Vec::new();
    let mut position_y: Vec<i64> = Vec::new();
    let mut velocity_x: Vec<i64> = Vec::new();
    let mut velocity_y: Vec<i64> = Vec::new();

    for s in split {
        let cap = re.captures(s).unwrap();
        let x: i64 = (&cap[1]).trim().parse().unwrap();
        let y: i64 = (&cap[2]).trim().parse().unwrap();
        let v_x: i64 = (&cap[3]).trim().parse().unwrap();
        let v_y: i64 = (&cap[4]).trim().parse().unwrap();

        position_x.push(x);
        position_y.push(y);
        velocity_x.push(v_x);
        velocity_y.push(v_y);
    }

    return (position_x, position_y, velocity_x, velocity_y);
}

fn get_distance(x: i64, y: i64, i: i64, j: i64) -> i64 {
    (x - i).abs() + (y - j).abs()
}

fn get_total_distance(position_x: &Vec<i64>, position_y: &Vec<i64>) -> i64 {
    let mut total_distance = 0;

    for i in 0..position_x.len() {
        for j in 0..position_x.len() {
            if i == j {
                continue;
            }
            total_distance += get_distance(position_x[i], position_y[i], position_x[j], position_y[j]);
        }
    }

    return total_distance;
}

fn coords_to_index(x: i64, y: i64, x_size: i64) -> i64 {
    return y * x_size + x;
}

fn draw_positions(position_x: &Vec<i64>, position_y: &Vec<i64>) {
    let min_x: i64 = *position_x.iter().min().unwrap();
    let max_x: i64 = *position_x.iter().max().unwrap();
    let min_y: i64 = *position_y.iter().min().unwrap();
    let max_y: i64 = *position_y.iter().max().unwrap();
    let x_size: i64 = max_x - min_x;

    let mut position_lookup: HashMap<i64, bool> = HashMap::new();

    for i in 0..position_x.len() {
        let index = coords_to_index(position_x[i] - min_x, position_y[i] - min_y, x_size);
        position_lookup.insert(index, true);
    }

    for y in min_y..=max_y {
        for x in min_x..=max_x {
            match position_lookup.get(&coords_to_index(x - min_x, y - min_y, x_size)) {
                Some(_) => print!("#"),
                None => print!(".")
            }
        }
        println!("");
    }
}

fn stars() {
    let (mut position_x, mut position_y, velocity_x, velocity_y) = get_data();
    let mut last_total_distance: i64 = std::i64::MAX;
    let mut j: i64 = 0;

    loop {
        let total_distance = get_total_distance(&position_x, &position_y);

        if total_distance > last_total_distance {
            // Step them back
            for i in 0..position_x.len() {
                position_x[i] -= velocity_x[i];
                position_y[i] -= velocity_y[i];
            }

            println!("seconds: {}", j - 1);
            draw_positions(&position_x, &position_y);
            return;
        }

        last_total_distance = total_distance;

        // Step them
        for i in 0..position_x.len() {
            position_x[i] += velocity_x[i];
            position_y[i] += velocity_y[i];
        }

        j += 1;
    }
}

fn main() {
    stars();
}
