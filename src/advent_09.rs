extern crate regex;
extern crate skiplist;
use std::fs::File;
use std::io::Read;
use regex::Regex;
use skiplist::SkipList;

fn mod_euc(dividend: i32, divisor: i32) -> i32 {
    ((dividend % divisor) + divisor) % divisor
}

fn get_data() -> (usize, usize) {
    let mut f = File::open("input/09.txt").expect("File not found");
    let mut contents = String::new();
    f.read_to_string(&mut contents).expect("File can't be read");
    let re = Regex::new(r"^(\d+) players; last marble is worth (\d+) points$").unwrap();
    let cap = re.captures(&contents).unwrap();
    let players: usize = (&cap[1]).parse().unwrap();
    let last_marble: usize = (&cap[2]).parse().unwrap();
    return (players, last_marble);
}

fn get_max_score(players: usize, last_marble: usize) {
    let mut scores: Vec<usize> = vec![0; players];
    let mut circle: SkipList<usize> = SkipList::new();
    circle.insert(0, 0);
    let mut current_marble: usize = 0;
    let mut current_player: usize = 0;

    for marble in 1..=last_marble {
        let len = circle.len();

        if marble % 23 == 0 {
            current_marble = mod_euc(current_marble as i32 - 7, len as i32) as usize;
            let removed_value = circle.remove(current_marble);
            scores[current_player] += marble + removed_value;
        } else {
            current_marble = (current_marble + 2) % len;
            circle.insert(marble, current_marble);
        }

        current_player = (current_player + 1) % players;
    }

    println!("{}", scores.iter().max().unwrap());
}

fn part_one() {
    let (players, last_marble) = get_data();
    get_max_score(players, last_marble);
}

fn part_two() {
    let (players, last_marble) = get_data();
    get_max_score(players, last_marble * 100);
}

fn main() {
    part_one();
    part_two();
}
