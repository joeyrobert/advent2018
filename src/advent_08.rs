use std::fs::File;
use std::io::Read;

fn sum_child_metadata(input: &Vec<usize>, start_pos: usize) -> (usize, usize) {
    let mut pos: usize = start_pos;
    let num_children = input[pos];
    pos += 1;
    let num_metadata = input[pos];
    pos += 1;

    let mut metadata: usize = 0;

    for _child in 0..num_children {
        let (child_metadata, child_pos) = sum_child_metadata(&input, pos);
        metadata += child_metadata;
        pos = child_pos;
    }

    for _metadatum in 0..num_metadata {
        metadata += input[pos];
        pos += 1;
    }

    return (metadata, pos);
}

fn get_data() -> Vec<usize> {
    let mut f = File::open("input/08.txt").expect("File not found");
    let mut contents = String::new();
    f.read_to_string(&mut contents).expect("File can't be read");
    return contents.split(" ").map(|s| s.parse().unwrap()).collect();
}

fn part_one() {
    let input: Vec<usize> = get_data();
    let pos: usize = 0;
    let (metadata, _end_pos) = sum_child_metadata(&input, pos);
    println!("{}", metadata);
}

fn sum_node_value(input: &Vec<usize>, start_pos: usize) -> (usize, usize) {
    let mut pos: usize = start_pos;
    let num_children = input[pos];
    pos += 1;
    let num_metadata = input[pos];
    pos += 1;

    let mut child_values: Vec<usize> = Vec::new();
    let mut value: usize = 0;

    if num_children > 0 {
        for _child in 0..num_children {
            let (child_value, child_pos) = sum_node_value(&input, pos);
            pos = child_pos;
            child_values.push(child_value);
        }

        for _metadatum in 0..num_metadata {
            let child = input[pos];
            if child <= child_values.len() {
                value += child_values[child - 1];
            }
            pos += 1;
        }

    } else {
        for _metadatum in 0..num_metadata {
            value += input[pos];
            pos += 1;
        }
    }

    return (value, pos);
}

fn part_two() {
    let input: Vec<usize> = get_data();
    let pos: usize = 0;
    let (metadata, _end_pos) = sum_node_value(&input, pos);
    println!("{}", metadata);
}

fn main() {
    part_one();
    part_two();
}
