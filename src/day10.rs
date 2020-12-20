use super::utils;

fn part1(input: &Vec<i32>) -> (usize, usize) {
    let mut a: usize = 0;
    let mut b: usize = 0;
    for i in 1..input.len() {
        let diff: i32 = input[i] - input[i - 1];
        if diff == 1 {
            a += 1;
        } else if diff == 3 {
            b += 1;
        }
    }
    return (a, b);
}

#[test]
fn test_part1() {
    let input: Vec<i32> = vec![0, 1, 4, 5, 6, 7, 10, 11, 12, 15, 16, 19];
    let result = part1(&input);
    assert_eq!(7, result.0);
    assert_eq!(4, result.1);
}

fn part2(input: &Vec<i32>) -> usize {
    println!("{:?}", input);
    let mut result: usize = 1;
    let mut n: usize = 1;
    for i in 1..input.len() {
        if input[i] - input[i - 1] == 3 {
            println!("{}", n);
            match n {
                3 => result *= 2,
                4 => result *= 4,
                5 => result *= 7,
                _ => {},
            }
            n = 1;
            continue;
        } else{
        // if input[i] - input[i - 1] == 1 {
            n += 1;
        }
    }
    return result;
}

#[test]
fn test_part2() {
    let input: Vec<i32> = vec![0, 1, 4, 5, 6, 7, 10, 11, 12, 15, 16, 19, 22];
    assert_eq!(8, part2(&input));
    let input2: Vec<i32> = vec![
        0, 1, 2, 3, 4, 7, 8, 9, 10, 11, 14, 17, 18, 19, 20, 23, 24, 25, 28, 31, 32, 33, 34, 35, 38,
        39, 42, 45, 46, 47, 48, 49, 52
    ];
    assert_eq!(19208, part2(&input2));
}

pub fn run() {
    println!("day10");

    let mut input: Vec<i32> = Vec::new();

    input.push(0);

    if let Ok(lines) = utils::read_lines("data/day10.txt") {
        for line in lines {
            if let Ok(v) = line {
                if let Ok(n) = v.parse::<i32>() {
                    input.push(n);
                }
            }
        }
    }

    input.sort();

    // part 1
    let result = part1(&input);
    println!("part1: {}", result.0 * (result.1 + 1));

    input.push(input[input.len()-1]+3);
    println!("part2: {}", part2(&input));
}
