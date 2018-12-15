use std::fs::File;
use std::io::Read;
use std::collections::{HashMap, HashSet};

fn coords_to_index(x: usize, y: usize) -> usize {
    (y * 150 + x) as usize
}

fn index_to_x(coords: usize) -> usize {
    coords % 150
}

fn index_to_y(coords: usize) -> usize {
    coords / 150
}

fn get_input() -> (Vec<char>, HashMap<usize, usize>, HashMap<usize, i32>) {
    let mut f = File::open("input/13.txt").expect("File not found");
    let mut contents = String::new();
    f.read_to_string(&mut contents).expect("File can't be read");
    let split = contents.split("\n");
    let mut track: Vec<char> = vec![' '; 150*150];
    let mut car_positions: HashMap<usize, usize> = HashMap::new();
    let mut car_directions: HashMap<usize, i32> = HashMap::new();
    let mut car_id: usize = 0;

    for (y, s) in split.enumerate() {
        for (x, mut ch) in s.chars().enumerate() {
            track[coords_to_index(x, y)] = match ch {
                'v' => {
                    car_directions.insert(car_id, 150);
                    '|'
                },
                '>' => {
                    car_directions.insert(car_id, 1);
                    '_'
                },
                '^' => {
                    car_directions.insert(car_id, -150);
                    '|'
                },
                '<' => {
                    car_directions.insert(car_id, -1);
                    '_'
                },
                x => x
            };

            if ch == 'v' || ch == '^' || ch == '>' || ch == '<' {
                car_positions.insert(car_id, coords_to_index(x, y));
                car_id += 1;
            }

        }
    }

    (track, car_positions, car_directions)
}

fn get_directions() -> (HashMap<(char, i32), i32>, HashMap<(usize, i32), i32>) {
    let mut slash_directions: HashMap<(char, i32), i32> = HashMap::new();
    slash_directions.insert(('/', 150), -1);
    slash_directions.insert(('\\', 150), 1);
    slash_directions.insert(('/', -150), 1);
    slash_directions.insert(('\\', -150), -1);
    slash_directions.insert(('/', 1), -150);
    slash_directions.insert(('\\', 1), 150);
    slash_directions.insert(('/', -1), 150);
    slash_directions.insert(('\\', -1), -150);

    let mut intersection_directions: HashMap<(usize, i32), i32> = HashMap::new();

    // left
    intersection_directions.insert((0, 150), 1);
    intersection_directions.insert((0, -150), -1);
    intersection_directions.insert((0, 1), -150);
    intersection_directions.insert((0, -1), 150);

    // straight
    intersection_directions.insert((1, 150), 150);
    intersection_directions.insert((1, -150), -150);
    intersection_directions.insert((1, 1), 1);
    intersection_directions.insert((1, -1), -1);

    // right
    intersection_directions.insert((2, 150), -1);
    intersection_directions.insert((2, -150), 1);
    intersection_directions.insert((2, 1), 150);
    intersection_directions.insert((2, -1), -150);

    (slash_directions, intersection_directions)
}

fn part_one() {
    let (track, mut car_positions, mut car_directions) = get_input();
    let (slash_directions, intersection_directions) = get_directions();
    let mut car_intersections: HashMap<usize, usize> = HashMap::new();

    for (car_id, _car_position) in &car_positions {
        car_intersections.insert(*car_id, 0); 
    }

    let mut tick = 0;
    loop {
        let mut car_ids: Vec<usize> = Vec::new();

        for (car_id, _car_position) in &car_positions {
            car_ids.push(*car_id)
        }

        car_ids.sort_unstable_by_key(|car_id| car_positions.get(car_id).unwrap());

        for car_id in car_ids {
            let next_position = (car_positions[&car_id] as i32 + car_directions[&car_id]) as usize;
            let mut next_direction = car_directions[&car_id];
            let next_track = track[next_position];

            match next_track {
                '+' => {
                    next_direction = *intersection_directions.get(&(car_intersections[&car_id], car_directions[&car_id])).unwrap();
                    let next_intersection = (car_intersections[&car_id] + 1) % 3;
                    car_intersections.insert(car_id, next_intersection); 
                },
                '/' | '\\' => {
                    next_direction = *slash_directions.get(&(next_track, car_directions[&car_id])).unwrap();
                }
                _ => {}
            }

            car_positions.insert(car_id, next_position);
            car_directions.insert(car_id, next_direction);

            let mut unique_positions: HashSet<usize> = HashSet::new();

            for (_car_id, car_position) in &car_positions {
                if unique_positions.contains(car_position) {
                    println!("{}: {},{}", tick, index_to_x(*car_position), index_to_y(*car_position));
                    return;
                } else {
                    unique_positions.insert(*car_position);
                }
            }
        }

        tick += 1;
    }
}

fn part_two() {
    let (track, mut car_positions, mut car_directions) = get_input();
    let (slash_directions, intersection_directions) = get_directions();
    let mut car_intersections: HashMap<usize, usize> = HashMap::new();

    for (car_id, _car_position) in &car_positions {
        car_intersections.insert(*car_id, 0); 
    }

    let mut tick = 0;
    loop {
        let mut car_ids: Vec<usize> = Vec::new();

        for (car_id, _car_position) in &car_positions {
            car_ids.push(*car_id)
        }

        car_ids.sort_unstable_by_key(|car_id| car_positions.get(car_id).unwrap());

        for car_id in car_ids {
            if !car_positions.contains_key(&car_id) {
                continue;
            }

            let next_position = (car_positions[&car_id] as i32 + car_directions[&car_id]) as usize;
            let mut next_direction = car_directions[&car_id];
            let next_track = track[next_position];

            match next_track {
                '+' => {
                    next_direction = *intersection_directions.get(&(car_intersections[&car_id], car_directions[&car_id])).unwrap();
                    let next_intersection = (car_intersections[&car_id] + 1) % 3;
                    car_intersections.insert(car_id, next_intersection); 
                },
                '/' | '\\' => {
                    next_direction = *slash_directions.get(&(next_track, car_directions[&car_id])).unwrap();
                }
                _ => {}
            }

            car_positions.insert(car_id, next_position);
            car_directions.insert(car_id, next_direction);

            let mut unique_positions: HashMap<usize, usize> = HashMap::new();
            let mut to_remove: Vec<usize> = Vec::new();

            for (car_id, car_position) in &car_positions {
                if unique_positions.contains_key(car_position) {
                    let other_car_id = unique_positions.get(car_position).unwrap();
                    to_remove.push(*car_id);
                    to_remove.push(*other_car_id);
                } else {
                    unique_positions.insert(*car_position, *car_id);
                }
            }

            for car_id in to_remove {
                car_positions.remove(&car_id);
                car_directions.remove(&car_id);
                car_intersections.remove(&car_id);
            }
        }

        if car_positions.len() == 1 {
            for (_car_id, car_position) in car_positions {
                println!("{}: {},{}", tick, index_to_x(car_position), index_to_y(car_position));
            }
            return;
        }

        tick += 1;
    }
}

fn main() {
    part_one();
    part_two();
}
