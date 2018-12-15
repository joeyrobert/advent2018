fn part_one() {
    let input = 864801;
    let mut scores: Vec<usize> = vec![3, 7];
    let mut elves: Vec<usize> = vec![0, 1];

    for _i in 0..(input+10) {
        let mut sum: usize = 0;
        for elf in &elves {
            sum += scores[*elf];
        }

        let digits: Vec<usize> = sum.to_string().chars().map(|d| d.to_digit(10).unwrap() as usize).collect();
        scores.extend(digits);

        for elf in elves.iter_mut() {
            *elf = (*elf + scores[*elf] + 1) % scores.len();
        }
    }

    for i in input..(input + 10) {
        print!("{}", scores[i]);
    }
    println!("");
}

fn part_two() {
    let input = vec![8, 6, 4, 8, 0, 1];
    // let input = vec![5,9,4,1,4];
    let input_len = input.len();
    let mut scores: Vec<usize> = vec![3, 7];
    let mut scores_len = 2;
    let mut elves: Vec<usize> = vec![0, 1];

    loop {
        let mut sum: usize = 0;
        for elf in &elves {
            sum += scores[*elf];
        }

        let mut digits: Vec<usize> = vec![];

        if sum == 0 {
            digits.push(0);
        }

        while sum > 0 {
            digits.push(sum % 10);
            sum /= 10;
        }

        digits.reverse();

        for digit in digits {
            scores.push(digit);
            scores_len += 1;

            let mut satisfied = true;

            if scores_len >= input_len {
                for (i, value) in input.iter().enumerate() {
                    let index = scores_len - input_len + i;
                    satisfied = satisfied && scores[index] == *value;
                }

                if satisfied {
                    println!("{:?}", scores_len - input_len);
                    return;
                }
            }
        }

        for elf in elves.iter_mut() {
            *elf = (*elf + scores[*elf] + 1) % scores_len;
        }
    }
}

fn main() {
    // part_one();
    part_two();
}
