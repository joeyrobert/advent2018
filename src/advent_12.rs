extern crate bit_vec;
extern crate regex;
use std::fs::File;
use std::io::Read;
use regex::Regex;
use std::collections::HashMap;
use bit_vec::BitVec;

fn part_one() {
    let mut f = File::open("input/12.txt").expect("File not found");
    let mut contents = String::new();
    f.read_to_string(&mut contents).expect("File can't be read");
    let split = contents.split("\n");
    let initial_re = Regex::new(r"initial state: ([#\.]+)").unwrap();
    let line_re = Regex::new(r"([#\.]+) => ([#\.])").unwrap();
    let mut state = BitVec::new();
    let mut transitions: HashMap<u8, bool> = HashMap::new();

    for (i, s) in split.enumerate() {
        match i {
            0 => {
                let cap = initial_re.captures(s).unwrap();
                state = str_to_state(&cap[1]);
            },
            1 => continue,
            _ => {
                let cap = line_re.captures(s).unwrap();
                transitions.insert(transition_to_u8(&cap[1]), &cap[2] == "#");
            }
        }
    }

    evolve_hash(&state, transitions);
}

fn str_to_state(initial_state: &str) -> BitVec {
    let mut state = BitVec::new();

    for ch in initial_state.chars() {
        match ch {
            '#' => {
                state.push(true);
            },
            '.' => {
                state.push(false);
            },
            _ => {}
        }
    }

    state
}

fn transition_to_u8(state: &str) -> u8 {
    let mut slice: u8 = 0;

    for (j, ch) in state.chars().enumerate() {
        match ch {
            '#' => {
                slice += 1 << j;
            },
            _ => {}
        }
    }

    slice
}

fn evolve_hash(initial_state: &BitVec, transitions: HashMap<u8, bool>) {
    let steps: i64 = 1000000;
    let mut state: HashMap<i32, bool> = HashMap::new();

    for (i, x) in initial_state.iter().enumerate() {
        if x {
            state.insert(i as i32, x);
        }
    }

    let mut min_i = *state.keys().min().unwrap() - 2;
    let mut max_i = *state.keys().max().unwrap() + 2;
    let mut last_total: i64 = 0;

    for step in 0..steps {
        let mut new_min_i = std::i32::MAX;
        let mut new_max_i = 0;
        let mut new_state: HashMap<i32, bool> = HashMap::new();

        for i in min_i..=max_i {
            let mut slice: u8 = 0;

            for j in -2..=2 {
                match state.get(&(j + i)) {
                    Some(value) => {
                        if *value {
                            slice += 1 << (j + 2);
                        }
                    },
                    None => {}
                }
            }

            match transitions.get(&slice) {
                Some(x) => {
                    if *x {
                        new_state.insert(i, true);
                        new_max_i = i + 2;

                        if i < new_min_i {
                            new_min_i = i - 2;
                        }
                    }
                },
                None => {}
            }

        }

        min_i = new_min_i;
        max_i = new_max_i;
        state = new_state;

        let mut total: i64 = 0;

        for (i, x) in &state {
            if *x {
                total += *i as i64;
            }
        }

        let eventual_value: i64 = 50000000000;
        let increment: i64 = 100000;
        if (step + 1) % increment == 0  {
            let delta = total - last_total;
            last_total = total;
            let remaining_steps = eventual_value - (step + 1);
            let possible_value = total + remaining_steps / increment * delta;
            println!("possible value at 50000000000: {}", possible_value);
        }
    }
}

fn main() {
    part_one();
}