extern crate regex;
use std::fs::File;
use std::io::Read;

fn coords_to_index(x: i32, y: i32) -> i32 {
    return y * 1000 + x;
}

fn get_distance(x: i32, y: i32, i: i32, j: i32) -> i32 {
    return (x - i).abs() + (y - j).abs();
}

fn part_one() {
    let mut f = File::open("input/06.txt").expect("File not found");
    let mut contents = String::new();
    f.read_to_string(&mut contents).expect("File can't be read");
    let split: Vec<&str> = contents.split("\n").collect();
    let mut min_x = 100000000;
    let mut max_x = 0;
    let mut min_y = 100000000;
    let mut max_y = 0;
    let mut values = vec![-1; 100000000];
    let mut points: Vec<(i32, i32)> = vec![];
    let mut point_sizes: Vec<i32> = vec![];
    let mut points_touch_edge: Vec<i32> = vec![];

    for s in &split {
        let cap: Vec<&str> = s.split(", ").collect();
        let x: i32 = (cap[0]).parse().unwrap();
        let y: i32 = (cap[1]).parse().unwrap();

        if x < min_x {
            min_x = x;
        }
        if y < min_y {
            min_y = y;
        }
        if x > max_x {
            max_x = x;
        }
        if y > max_y {
            max_y = y;
        }

        points.push((x, y));
        point_sizes.push(0);
    }

    for x in min_x..=max_x {
        for y in min_y..=max_y {
            let mut min_point: i32 = -1;
            let mut min_distance: i32 = 100000000;
            let mut distances: Vec<i32> = vec![];

            for i in 0..points.len() {
                let distance = get_distance(x, y, points[i].0, points[i].1);
                distances.push(distance);
                if distance < min_distance {
                    min_point = i as i32;
                    min_distance = distance;
                }
            }

            let min_distances: Vec<&i32> = distances.iter().filter(|&&distance| distance == min_distance).collect();
            let min_distances_length = min_distances.len();

            if min_distances_length == 1 {
                values[coords_to_index(x, y) as usize] = min_point;
                point_sizes[min_point as usize] += 1;

                if x == min_x || x == max_x || y == min_y || y == max_y {
                    points_touch_edge.push(min_point);
                }
            }
        }
    }

    points_touch_edge.sort();
    points_touch_edge.dedup();
    points_touch_edge.reverse();
    for i in points_touch_edge {
        point_sizes.remove(i as usize);
    }

    println!("{}", point_sizes.iter().max().unwrap());
}

fn part_two() {
    let mut f = File::open("input/06.txt").expect("File not found");
    let mut contents = String::new();
    f.read_to_string(&mut contents).expect("File can't be read");
    let split: Vec<&str> = contents.split("\n").collect();
    let mut points: Vec<(i32, i32)> = vec![];

    for s in &split {
        let cap: Vec<&str> = s.split(", ").collect();
        let x: i32 = (cap[0]).parse().unwrap();
        let y: i32 = (cap[1]).parse().unwrap();
        points.push((x, y));
    }

    let mut region_size = 0;

    for x in 0..1000 {
        for y in 0..1000 {
            let mut distance = 0;

            for point in &points {
                distance += get_distance(x, y, point.0, point.1);
            }

            if distance < 10000 {
                region_size += 1;
            }
        }
    }

    println!("{}", region_size);
}

fn main() {
    part_one();
    part_two();
}
