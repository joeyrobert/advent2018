extern crate regex;
use std::fs::File;
use std::io::Read;
use std::collections::HashSet;
use std::collections::HashMap;
use regex::Regex;

fn get_data() -> (HashSet<String>, HashMap<String, Vec<String>>) {
    let mut f = File::open("input/07.txt").expect("File not found");
    let mut contents = String::new();
    f.read_to_string(&mut contents).expect("File can't be read");
    let split = contents.split("\n");
    let re = Regex::new(r"^Step (.*?) must be finished before step (.*?) can begin.$").unwrap();
    let mut dependencies: Vec<(String, String)> = vec![];
    let mut full_dependencies: HashMap<String, Vec<String>> = HashMap::new();
    let mut steps = HashSet::new();

    for s in split {
        let cap = re.captures(s).unwrap();
        dependencies.push((cap[1].to_string(), cap[2].to_string()));
        full_dependencies.entry(cap[1].to_string()).or_insert(vec![]);
        full_dependencies.entry(cap[2].to_string()).or_insert(vec![]).push(cap[1].to_string());
        steps.insert(cap[1].to_string());
        steps.insert(cap[2].to_string());
    }

    return (steps, full_dependencies);
}

fn get_ready_steps(completed: HashSet<String>, full_dependencies: HashMap<String, Vec<String>>) -> Vec<String> {
    let mut ready_steps: Vec<String> = vec![];
    for (step, dependencies) in full_dependencies.clone() {
        if !completed.contains(&step) {
            let mut satisfied = true;
            for dependency in dependencies {
                satisfied = satisfied && completed.contains(&dependency);
            }

            if satisfied {
                ready_steps.push(step);
            }
        }
    }

    ready_steps.sort();
    return ready_steps;
}

fn get_duration_for_step(step: String) -> i32 {
    let chars: Vec<char> = step.chars().collect();
    return (chars[0] as i32) - 4;
}

fn part_one() {
    let (steps, full_dependencies) = get_data();
    let mut completed: HashSet<String> = HashSet::new();
    let mut order: Vec<String> = vec![];

    while order.len() < steps.len() {
        let ready_steps: Vec<String> = get_ready_steps(completed.clone(), full_dependencies.clone());
        order.push(ready_steps[0].clone());
        completed.insert(ready_steps[0].clone());
    }

    println!("{}", order.join(""));
}

fn part_two() {
    let (steps, full_dependencies) = get_data();
    let mut completed: HashSet<String> = HashSet::new();
    let mut order: Vec<String> = vec![];
    let mut queued: HashSet<String> = HashSet::new();
    let mut work_queue: Vec<String> = vec![];
    let mut time_spent: HashMap<String, i32> = HashMap::new();
    let mut time = 0;

    while order.len() < steps.len() {
        let ready_steps: Vec<String> = get_ready_steps(completed.clone(), full_dependencies.clone());

        for step in ready_steps {
            if !queued.contains(&step) {
                work_queue.push(step.clone());
                queued.insert(step.clone());
            }
        }

        let mut remove_from_work_queue: Vec<usize> = vec![];

        for i in 0..5 {
            if work_queue.len() > i {
                let step = (&work_queue[i]).to_string();
                *time_spent.entry(step.to_string()).or_insert(0) += 1;

                if *time_spent.get(&step.to_string()).unwrap() == get_duration_for_step(step.to_string()) {
                    remove_from_work_queue.push(i);
                    completed.insert(step.clone());
                    order.push(step.clone());
                }
            }
        }

        for i in remove_from_work_queue {
            work_queue.remove(i);
        }

        time += 1;
    }

    println!("{}", time);
}

fn main() {
    part_one();
    part_two();
}
