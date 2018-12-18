use std::fs::File;
use std::io::Read;
use std::collections::HashMap;
use std::collections::HashSet;
use std::time::Duration;
use std::thread;

static MAX_DISTANCE: usize = 100000;

fn coords_to_index(x: usize, y: usize) -> usize {
    (y * 32 + x) as usize
}

fn index_to_x(coords: usize) -> usize {
    coords % 32
}

fn index_to_y(coords: usize) -> usize {
    coords / 32
}

fn get_neighbors(open_spots: &Vec<usize>, spot: usize) -> Vec<usize> {
    let x = index_to_x(spot);
    let y = index_to_y(spot);
    let mut neighbors = vec![
        coords_to_index(x, y - 1),
        coords_to_index(x - 1, y),
        coords_to_index(x + 1, y),
        coords_to_index(x, y + 1)
    ];
    neighbors = neighbors.into_iter().filter(|index| open_spots.contains(&index)).collect::<Vec<usize>>();
    neighbors
}

fn part_one() {
    let mut f = File::open("input/15.txt").expect("File not found");
    let mut contents = String::new();
    f.read_to_string(&mut contents).expect("File can't be read");
    let split = contents.split("\n");
    let mut open_spots: Vec<usize> = Vec::new();
    let mut elves: HashSet<usize> = HashSet::new();
    let mut gnomes: HashSet<usize> = HashSet::new();
    let mut hitpoints: HashMap<usize, i32> = HashMap::new();
    let mut positions: HashMap<usize, usize> = HashMap::new();
    let mut id: usize = 0;

    for (y, s) in split.enumerate() {
        for (x, ch) in s.chars().enumerate() {
            let position = coords_to_index(x, y);

            match ch {
                'G' => {
                    positions.insert(id, position);
                    hitpoints.insert(id, 200);
                    gnomes.insert(id);
                    open_spots.push(position);
                    id += 1;
                },
                'E' => {
                    positions.insert(id, position);
                    hitpoints.insert(id, 200);
                    elves.insert(id);
                    open_spots.push(position);
                    id += 1;
                },
                '.' => {
                    open_spots.push(position);
                },
                _ => {}
            }
        }
    }

    let mut step = 0;

    loop {
        let mut positions_copy = positions.clone();
        let mut positions_vec: Vec<_> = positions_copy.iter().collect();
        positions_vec.sort_by_key(|position| position.1);
        let mut removed_enemies: HashSet<usize> = HashSet::new();

        for (id, position) in &positions_vec {
            if removed_enemies.contains(id) {
                continue;
            }

            let positions_copy_two = positions.clone();
            let mut occupied_spots: HashSet<usize> = HashSet::new();
            for (_, position) in &positions_copy_two {
                occupied_spots.insert(*position);
            }

            let mut unoccupied_spots: Vec<usize> = open_spots.iter().filter(|x| !occupied_spots.contains(x)).cloned().collect();

            // println!("IS IT FILTER? {} = {}", open_spots.len(), unoccupied_spots.len());
            let (open_distances, _) = get_distances(&open_spots, **position);
            let (distances, previous) = get_distances(&unoccupied_spots, **position);



            println!("unoccupied_spots: {:#?}", unoccupied_spots);

            // find enemies
            let is_elf = elves.contains(id);

            let enemies: Vec<_> = positions_copy_two
                .iter()
                .filter(|(enemy_id, _)| elves.contains(enemy_id) != is_elf)
                .collect();

            // println!("enemies: {:#?}", enemies);

            let mut enemies_in_range: Vec<_> = enemies
                .iter()
                .filter(|enemy| distances[enemy.1] == 1)
                .collect();

            // sort in reading order
            enemies_in_range.sort_by_key(|(enemy_id, _)| enemy_id);

            let new_position = if enemies_in_range.len() == 0 {
                // find squares adjacent to enemies
                let mut enemy_neighbors: Vec<_> = enemies
                    .iter()
                    .flat_map(|(enemy_id, enemy_position)| get_neighbors(&unoccupied_spots, **enemy_position))
                    .collect();

                if enemy_neighbors.len() > 0 {
                    // find nearest adjacent square
                    enemy_neighbors.sort_by_key(|neighbor_position| distances[neighbor_position] * 10000 + neighbor_position);
                    let target = enemy_neighbors[0];

                    // move one towards the target
                    let (target_distances, target_previous) = get_distances(&unoccupied_spots, target);
                    positions.insert(**id, target_previous[position]);

                    println!("{:#?}", unoccupied_spots);
                    println!("moving ({},{}) -> {} ({},{})", index_to_x(**position), index_to_y(**position), target_previous[position], index_to_x(target_previous[position]), index_to_y(target_previous[position]));

                    enemies_in_range = enemies
                        .iter()
                        .filter(|enemy| target_distances[&target_previous[position]] == 1)
                        .collect();

                    target_previous[position]
                } else {
                    **position
                }
            } else {
                **position
            };

            if enemies_in_range.len() > 0 {
                let (enemy_id, enemy_position) = enemies_in_range[0];
                let next_hitpoint = hitpoints[enemy_id] - 3;
                hitpoints.insert(**enemy_id, next_hitpoint);

                // println!("{:#?}", hitpoints);

                if hitpoints[enemy_id] <= 0 {
                    positions.remove(enemy_id);
                    hitpoints.remove(enemy_id);
                    removed_enemies.insert(**enemy_id);

                    // println!("removing enemy {} ({})", enemy_id, elves.contains(enemy_id));
                }
            }
        }


        // println!("Incrementing step");
        step += 1;
        let mut positions_copy = positions.clone();
        let mut num_elves = 0;
        let mut num_gnomes = 0;
        for (id, _) in positions_copy {
            if elves.contains(&id) {
                num_elves += 1;
            } else if gnomes.contains(&id) {
                num_gnomes += 1;
            }
        }

        if num_elves == 0 || num_gnomes == 0 {
            let mut total_hitpoints = 0;
            for (_, hp) in hitpoints {
                total_hitpoints += hp;
            }
            // println!("{} * {} = {}", step, total_hitpoints, step * total_hitpoints);
            break;
        }

        draw(step, &positions, &open_spots, &elves);
        thread::sleep(Duration::from_millis(250))
    }
}

fn draw(step: usize, positions: &HashMap<usize, usize>, open_spots: &Vec<usize>, elves: &HashSet<usize>) {
    let mut id_at_position: HashMap<usize, usize> = HashMap::new();
    for (id, position) in positions {
        id_at_position.insert(position.clone(), id.clone());
    }
    // print!("{}[2J", 27 as char);
    println!("step: {}", step);

    for y in 0..7 {
        for x in 0..7 {
            let position = coords_to_index(x, y);

            if id_at_position.contains_key(&position) {
                let id = id_at_position[&position];
                if elves.contains(&id) {
                    print!("E");
                } else {
                    print!("G");
                }
            } else if open_spots.contains(&position) {
                print!(".")
            } else {
                print!("#");
            }
        }
        println!("");
    }
}

// slow dijkstra algorithm, uses sorting instead of binaryheap
fn get_distances(unoccupied_spots: &Vec<usize>, source: usize) -> (HashMap<usize, usize>, HashMap<usize, usize>) {
    let mut distances: HashMap<usize, usize> = HashMap::new();
    let mut previous: HashMap<usize, usize> = HashMap::new();
    let mut queue: Vec<usize> = Vec::new();

    for spot in unoccupied_spots {
        distances.insert(*spot, MAX_DISTANCE);
        queue.push(*spot);
    }

    distances.insert(source, 0);

    queue.sort_by_key(|position| distances[&position]);

    while queue.len() > 0 {
        let spot = queue.remove(0);
        let neighbors = get_neighbors(unoccupied_spots, spot);
        let distance = distances[&spot] + 1;

        for neighbor in neighbors {
            if distance < distances[&neighbor] {
                distances.insert(neighbor, distance);
                previous.insert(neighbor, spot);
            }
        }

        queue.sort_by_key(|position| distances[&position]);
    }

    (distances, previous)

}

fn main() {
    part_one();
}
