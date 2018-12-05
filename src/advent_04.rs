extern crate regex;
use std::fs::File;
use std::io::Read;
use regex::Regex;
use std::collections::HashMap;

fn day_in_year(month: i32, day: i32) -> i32 {
    // let month_length = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
    let month_offset = [0, 31, 59, 90, 120, 151, 181, 212, 242, 273, 303, 334];
    return month_offset[month as usize] + day - 1;
}

fn minute_in_day(hour: i32, minute: i32) -> i32 {
    return hour * 60 + minute;
}

fn minute_in_year(month: i32, day: i32, hour: i32, minute: i32) -> i32 {
    return day_in_year(month, day) * 24 * 60 + minute_in_day(hour, minute);
}

fn minute_in_year_to_minute_in_hour(minutes: i32) -> i32 {
    return minutes % 60;
}

fn get_data() -> (HashMap<i32, i32>, HashMap<i32, HashMap<i32, i32>>) {
    let mut f = File::open("input/04.txt").expect("File not found");
    let mut contents = String::new();
    f.read_to_string(&mut contents).expect("File can't be read");
    let mut split: Vec<&str> = contents.split("\n").collect();
    split.sort_unstable();
    let line_re = Regex::new(r"^\[((\d{4})\-(\d{2})\-(\d{2}) (\d{2}):(\d{2}))\] (.*?)$").unwrap();
    let guard_re = Regex::new(r"^Guard #(\d+) begins shift$").unwrap();

    let mut total_minutes: HashMap<i32, i32> = HashMap::new();
    let mut likely_minutes: HashMap<i32, HashMap<i32, i32>> = HashMap::new();
    let mut active_guard = 0;
    let mut start_minute = 0;

    for s in &split {
        if s.len() > 0 {
            let cap = line_re.captures(s).unwrap();
            let month: i32 = (&cap[3]).parse().unwrap();
            let day: i32 = (&cap[4]).parse().unwrap();
            let hour: i32 = (&cap[5]).parse().unwrap();
            let minute: i32 = (&cap[6]).parse().unwrap();
            let action = &cap[7];

            if action.starts_with("Guard") {
                let guard_cap = guard_re.captures(action).unwrap();
                active_guard = (&guard_cap[1]).parse().unwrap();
            } else if action.starts_with("falls") {
                start_minute = minute_in_year(month, day, hour, minute);
            } else if action.starts_with("wakes") {
                let end_minute = minute_in_year(month, day, hour, minute);
                let sleep_duration = end_minute - start_minute;
                *total_minutes.entry(active_guard).or_insert(0) += sleep_duration;
                let mut likely_minutes_for_guard = likely_minutes.entry(active_guard).or_insert(HashMap::new());

                for minute in start_minute..end_minute {
                    *likely_minutes_for_guard.entry(minute_in_year_to_minute_in_hour(minute)).or_insert(0) += 1;
                }
            }
        }
    }

    return (total_minutes, likely_minutes);
}

fn part_one() {
    let (total_minutes, likely_minutes) = get_data();
    let mut max_guard: i32 = 0;
    let mut max_minutes: i32 = 0;
    let mut max_occurrences: i32 = 0; 
    let mut max_minute_in_day: i32 = 0;

    for (guard, minutes) in total_minutes {
        if minutes > max_minutes {
            max_guard = guard;
            max_minutes = minutes;
        }
    }

    let likely_minutes_value = likely_minutes.get(&max_guard).unwrap();

    for (minute_in_day, occurrences) in likely_minutes_value {
        if *occurrences > max_occurrences {
            max_occurrences = *occurrences;
            max_minute_in_day = *minute_in_day;
        }
    }
    println!("{} * {} = {}", max_guard, max_minute_in_day, max_guard * max_minute_in_day);
}

fn part_two() {
    let (total_minutes, likely_minutes) = get_data();

    let mut max_guard: i32 = 0;
    let mut max_minute_in_day: i32 = 0;
    let mut max_occurrences: i32 = 0; 

    for (guard, _minutes) in total_minutes {
        let likely_minutes_value = likely_minutes.get(&guard).unwrap();

        for (minute_in_day, occurrences) in likely_minutes_value {
            if *occurrences > max_occurrences {
                max_guard = guard;
                max_occurrences = *occurrences;
                max_minute_in_day = *minute_in_day;
            }
        }
    }

    println!("{} * {} = {}", max_guard, max_minute_in_day, max_guard * max_minute_in_day);
}

fn main() {
    part_one();
    part_two();
}
