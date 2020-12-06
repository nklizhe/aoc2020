use super::utils;

fn day1_part1(input: &Vec<i32>) -> i32 {
    for i in 0..input.len() - 1 {
        for j in i + 1..input.len() {
            if input[i] + input[j] == 2020 {
                return input[i] * input[j];
            }
        }
    }
    return 0;
}

fn day1_part2(input: &Vec<i32>) -> i32 {
    for i in 0..input.len() - 2 {
        for j in i + 1..input.len() - 1 {
            for k in j + 1..input.len() {
                if input[i] + input[j] + input[k] == 2020 {
                    return input[i] * input[j] * input[k];
                }
            }
        }
    }
    return 0;
}

pub fn run() {
    println!("day1");

    let mut input: Vec<i32> = Vec::new();

    if let Ok(lines) = utils::read_lines("data/day1.txt") {
        for line in lines {
            if let Ok(v) = line {
                if let Ok(n) = v.parse::<i32>() {
                    input.push(n);
                }
            }
        }
    }

    // part 1
    println!("{}", day1_part1(&input));

    // part 2
    println!("{}", day1_part2(&input));
}
