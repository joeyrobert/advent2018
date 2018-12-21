extern crate regex;
use std::fs::File;
use std::io::Read;
use regex::Regex;
use std::collections::HashSet;

fn part_one() {
    let mut f = File::open("input/17.txt").expect("File not found");
    let mut contents = String::new();
    f.read_to_string(&mut contents).expect("File can't be read");
    let split = contents.split("\n");
    let re = Regex::new(r"([y|x])=(\d+), ([y|x])=(\d+)..(\d+)").unwrap();
    let mut clay: HashSet<(i32, i32)> = HashSet::new();
    let mut water: HashSet<(i32, i32)> = HashSet::new();
    let mut falling: HashSet<(i32, i32)> = HashSet::new();

    for s in split {
        let cap = re.captures(s).unwrap();
        let line_type: &str = &cap[1];
        let line_position: i32 = (&cap[2]).parse().unwrap();
        let range_start: i32 = (&cap[4]).parse().unwrap();
        let range_end: i32 = (&cap[5]).parse().unwrap();

        for position in range_start..=range_end {
            if line_type == "y" {
                clay.insert((position, line_position));
            } else {
                clay.insert((line_position, position));
            }
        }
    }

    let mut range = determine_range(&clay, &water, &falling);

    // full range has extra x on each side:
    range = (range.0 - 1, range.1 + 1, range.2, range.3);

    // flood fill
    let spring = (500, 1);
    flood_fill(spring, &clay, &mut water, &mut falling, range);

    println!("{}", water.len());
}

fn determine_range(clay: &HashSet<(i32, i32)>, water: &HashSet<(i32, i32)>, falling: &HashSet<(i32, i32)>) -> (i32, i32, i32, i32) {
    let mut min_y = std::i32::MAX;
    let mut max_y = 0;
    let mut min_x = std::i32::MAX;
    let mut max_x = 0;
    let mut union: HashSet<_> = clay.union(water).collect();

    for (x, y) in union {
        if *x < min_x {
            min_x = *x;
        }

        if *x > max_x {
            max_x = *x;
        }

        if *y < min_y {
            min_y = *y;
        }

        if *y > max_y {
            max_y = *y;
        }
    }
    (min_x, max_x, min_y, max_y)
}

fn draw(clay: &HashSet<(i32, i32)>, water: &HashSet<(i32, i32)>, falling: &HashSet<(i32, i32)>) {
    let (min_x, max_x, min_y, max_y) = determine_range(clay, water, falling);

    for y in min_y..=max_y {
        for x in min_x..=max_x {
            if x == 500 && y == 0 {
                print!("+");
            } else if clay.contains(&(x, y)) {
                print!("#");
            } else if water.contains(&(x, y)) {
                print!("~");
            } else if falling.contains(&(x, y)) {
                print!("|");
            } else {
                print!(".");
            }
        }
        println!("");
    }
    println!("");
}

fn flood_fill(node: (i32, i32), clay: &HashSet<(i32, i32)>, water: &mut HashSet<(i32, i32)>, falling: &mut HashSet<(i32, i32)>, range: (i32, i32, i32, i32)) {
    if node.0 < range.0 || node.0 > range.1 || node.1 < range.2 || node.1 > range.3 {
        return;
    }

    println!("inserting {:?}", node);
    (*falling).insert(node);
    draw(clay, water, falling);

    let below = (node.0, node.1 + 1);
    let left = (node.0 - 1, node.1);
    let right = (node.0 + 1, node.1);

    // fill below
    if !clay.contains(&below) && !water.contains(&below) {
        println!("below");
        flood_fill(below, clay, water, falling, range);
    }

    // fill left/right
    if !clay.contains(&left) && !water.contains(&left) && !falling.contains(&left) && (clay.contains(&below) || water.contains(&below)) {
        println!("left");
        flood_fill(left, clay, water, falling, range);
    }

    if !clay.contains(&right) && !water.contains(&right) && !falling.contains(&right) && (clay.contains(&below) || water.contains(&below)) {
        println!("right");
        flood_fill(right, clay, water, falling, range);
    }

    println!("{:?}: has walls? {}", &node, has_both_walls(node, clay, water, falling));
    if has_both_walls(node, clay, water, falling) {
        fill_level(node, clay, water);
    }
}


// fn has_walls(node: (i32, i32), clay: &HashSet<(i32, i32)>, water: &HashSet<(i32, i32)>) -> bool {
//     let directions = [-1, 1];
//     let mut has_both = true;

//     for direction in directions.iter() {
//         let mut x = node.0;

//         loop {
//             let node = (x, node.1);
//             if clay.contains(&node) {
//                 has_both = has_both && true;
//                 break;
//             } else if !clay.contains(&node) && !water.contains(&node) {
//                 return false;
//             }

//             x += direction;
//         }
//     }

//     has_both
// }

// fn fill_line(node: (i32, i32), clay: &HashSet<(i32, i32)>, water: &mut HashSet<(i32, i32)>) {
//     let directions = [-1, 1];

//     for direction in directions.iter() {
//         let mut x = node.0;
//         loop {
//             let node = (x, node.1);
//             if clay.contains(&node) {
//                 break;
//             }
//             water.insert(node);
//             x += direction;
//         }
//     }
// }
fn has_both_walls(node: (i32, i32), clay: &HashSet<(i32, i32)>, water: &HashSet<(i32, i32)>, falling: &HashSet<(i32, i32)>) -> bool {
    has_wall(node, clay, water, falling, 1) && has_wall(node, clay, water, falling, 1)
}

fn has_wall(node: (i32, i32), clay: &HashSet<(i32, i32)>, water: &HashSet<(i32, i32)>, falling: &HashSet<(i32, i32)>, offset: i32) -> bool {
    let mut x = node.0;
    loop {
        if (clay.contains(&(x, node.1)) && water.contains(&(x, node.1)) && falling.contains(&(x, node.1))) {
            return false;
        }
        if (clay.contains(&(x, node.1))) {
             return true;  
        }
        x += offset;
    }
}

fn fill_level(node: (i32, i32), clay: &HashSet<(i32, i32)>, water: &mut HashSet<(i32, i32)>) {
    fill_side(node, clay, water, 1);
    fill_side(node, clay, water, -1);
}

fn fill_side(node: (i32, i32), clay: &HashSet<(i32, i32)>, water: &mut HashSet<(i32, i32)>, offset: i32) {
    let mut x = node.0;
    loop {
        if (clay.contains(&(x, node.1))) {
            return;  
        }

        water.insert((x, node.1));
        x += offset;
    }
}

fn main() {
    part_one();
}
