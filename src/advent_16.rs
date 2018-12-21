extern crate regex;
use std::fs::File;
use std::io::Read;
use std::collections::HashMap;
use std::collections::HashSet;
use regex::Regex;

const OPCODES: [&'static str; 16] = [
    "addr",
    "addi",
    "mulr",
    "muli",
    "banr",
    "bani",
    "borr",
    "bori",
    "setr",
    "seti",
    "gtir",
    "gtri",
    "gtrr",
    "eqir",
    "eqri",
    "eqrr"
];

fn get_data() -> (Vec<Vec<usize>>, Vec<Vec<usize>>, Vec<Vec<usize>>, Vec<Vec<usize>>) {
    let mut f = File::open("input/16.txt").expect("File not found");
    let mut contents = String::new();
    f.read_to_string(&mut contents).expect("File can't be read");
    let split: Vec<_> = contents.split("\n").collect();
    let mut split_iter = split.iter();
    let inst_re = Regex::new(r"^(\d+) (\d+) (\d+) (\d+)$").unwrap();
    let registers_re = Regex::new(r"\[(\d+), (\d+), (\d+), (\d+)\]").unwrap();

    let mut befores: Vec<_> = Vec::new();
    let mut insts: Vec<_> = Vec::new();
    let mut afters: Vec<_> = Vec::new();
    let mut programs: Vec<_> = Vec::new();

    loop {
        let mut s = match split_iter.next() {
            Some(s) => s,
            None => break
        };

        if s.starts_with("Before") {
            let before_captures = registers_re.captures(s).unwrap();
            s = split_iter.next().unwrap();
            let inst_captures = inst_re.captures(s).unwrap();
            s = split_iter.next().unwrap();
            let after_captures = registers_re.captures(s).unwrap();
            split_iter.next();

            let mut before: Vec<usize> = Vec::new();
            let mut after: Vec<usize> = Vec::new();
            let mut inst: Vec<usize> = Vec::new();

            for i in 1..5 {
                before.push((before_captures[i]).parse().unwrap());
                inst.push((inst_captures[i]).parse().unwrap());
                after.push((after_captures[i]).parse().unwrap());
            }

            befores.push(before);
            insts.push(inst);
            afters.push(after);
        } else {
            match inst_re.captures(s) {
                Some(captures) => {
                    let mut program: Vec<usize> = Vec::new();
                    for i in 1..5 {
                        program.push((captures[i]).parse().unwrap());
                    }
                    programs.push(program);
                },
                None => {}
            }
        }
    }

    (befores, insts, afters, programs)
}

fn register_equality(a: &Vec<usize>, b: &Vec<usize>) -> bool {
    let mut equal = true;
    for index in 0..4 {
        equal = equal && a[index] == b[index];
    }
    equal
}

fn part_one() {
    let (befores, insts, afters, _) = get_data();
    let mut total = 0;

    for (i, inst) in insts.iter().enumerate() {
        let mut count = 0;

        for opcode in &OPCODES {
            let computed_after = step(&befores[i], opcode, inst[1], inst[2], inst[3]);
            let equal = register_equality(&afters[i], &computed_after);

            if equal {
                count += 1;
            }
        }

        if count >= 3 {
            total += 1;
        }
    }

    println!("{}", total);
}

fn part_two() {
    let (befores, insts, afters, programs) = get_data();
    let mut num_to_opcodes: HashMap<usize, Vec<&str>> = HashMap::new();
    let mut num_to_opcode: HashMap<usize, &str> = HashMap::new();

    for (i, inst) in insts.iter().enumerate() {
        let mut possible: Vec<&str> = Vec::new();
        for opcode in &OPCODES {
            let computed_after = step(&befores[i], opcode, inst[1], inst[2], inst[3]);
            let equal = register_equality(&afters[i], &computed_after);

            if equal {
                possible.push(&opcode);
            }
        }

        if num_to_opcodes.contains_key(&inst[0]) {
            let previous_possible = num_to_opcodes.get(&inst[0]).unwrap().to_vec();
            let mut new_possible: Vec<&str> = Vec::new();

            for opcode in previous_possible {
                if possible.contains(&opcode) {
                    new_possible.push(opcode);
                }
            }

            num_to_opcodes.insert(inst[0], (*new_possible).to_vec());
        } else {
            num_to_opcodes.insert(inst[0], (*possible).to_vec());
        }
    }

    let mut spoken_for: HashSet<&str> = HashSet::new();
    
    while num_to_opcode.len() < 16 {
        for (num, opcodes) in num_to_opcodes.iter() {
            let filtered_opcodes: Vec<_> = opcodes.iter().filter(|&opcode| !spoken_for.contains(opcode)).collect();

            if filtered_opcodes.len() == 1 {
                num_to_opcode.insert(*num, filtered_opcodes[0]);
                spoken_for.insert(filtered_opcodes[0]);
            }
        }
    }

    let mut registers: Vec<usize> = vec![0, 0, 0, 0];
    for inst in programs {
        let opcode = num_to_opcode[&inst[0]];
        registers = step(&registers, opcode, inst[1], inst[2], inst[3]);
    }

    println!("registers: {:?}", registers);

}


fn step(before: &Vec<usize>, opcode: &str, a: usize, b: usize, c: usize) -> Vec<usize> {
    let mut after: Vec<usize> = Vec::new();

    for i in 0..4 {
        after.push(before[i]);
    }

    after[c] = match opcode {
        "addr" => before[a] + before[b],
        "addi" => before[a] + b,
        "mulr" => before[a] * before[b],
        "muli" => before[a] * b,
        "banr" => before[a] & before[b],
        "bani" => before[a] & b,
        "borr" => before[a] | before[b],
        "bori" => before[a] | b,
        "setr" => before[a],
        "seti" => a,
        "gtir" => if a > before[b] { 1 } else { 0 },
        "gtri" => if before[a] > b { 1 } else { 0 },
        "gtrr" => if before[a] > before[b] { 1 } else { 0 },
        "eqir" => if a == before[b] { 1 } else { 0 },
        "eqri" => if before[a] == b { 1 } else { 0 },
        "eqrr" => if before[a] == before[b] { 1 } else { 0 },
        _ => before[c]
    };

    after
}

fn main() {
    part_one();
    part_two();
}
