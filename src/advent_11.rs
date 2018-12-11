fn coords_to_index(x: i32, y: i32) -> usize {
    ((y - 1) * 300 + x - 1) as usize
}

fn power_at_x_y(x: i32, y: i32, serial_number: i32) -> i32 {
    ((x + 10) * y + serial_number) * (x + 10) / 100 % 10 - 5
}

fn part_one() {
    let mut max_power = 0;
    let mut max_x = 0;
    let mut max_y = 0;

    for start_y in 1..=298 {
        for start_x in 1..=298 {
            let mut this_power = 0;
            for x in start_x..(start_x + 3) {
                for y in start_y..(start_y + 3) {
                    this_power += power_at_x_y(x, y, 6548);
                }
            }

            if this_power > max_power {
                max_power = this_power;
                max_x = start_x;
                max_y = start_y;
            }
        }
    }

    println!("{},{} = {}", max_x, max_y, max_power);
}

fn part_two() {
    let mut powers: Vec<i32> = vec![0; 300 * 300];

    for x in 1..=300 {
        for y in 1..=300 {
            powers[coords_to_index(x, y)] = power_at_x_y(x, y, 6548);
        }
    }

    let mut max_power = 0;
    let mut max_x = 0;
    let mut max_y = 0;
    let mut max_square_size = 0;

    for square_size in 1..300 {
        for start_y in 1..=(300 - square_size + 1) {
            for start_x in 1..=(300 - square_size + 1) {
                let mut this_power = 0;
                for x in start_x..(start_x + square_size) {
                    for y in start_y..(start_y + square_size) {
                        this_power += powers[coords_to_index(x, y)];
                    }
                }

                if this_power > max_power {
                    max_power = this_power;
                    max_x = start_x;
                    max_y = start_y;
                    max_square_size = square_size;
                }
            }
        }
    }

    println!("{},{},{} = {}", max_x, max_y, max_square_size, max_power);
}

fn main() {
    part_one();
    part_two();
}
