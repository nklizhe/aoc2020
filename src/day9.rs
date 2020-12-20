use super::utils;
use std::i64;

fn check(input: &Vec<i64>, start: usize, n: usize) -> bool {
    let v :i64 = input[start+n];
    for i in start..start+n-1 {
        for j in i..start+n {
            if input[i]+input[j] == v {
                return true;
            }
        }
    }
    return false;
}

fn part1(input:&Vec<i64>) -> i64 {
    let n:usize = 25;
    for i in 0..input.len()-n-1 {
        if !check(input, i, n) {
            return input[i+n];
        }
    }
    return -1;
}

fn part2(input:&Vec<i64>, v:i64) -> (i64, i64) {
    for i in 0..input.len()-2{
        let mut sum:i64 = input[i];
        let mut j:usize = i;
        let mut min = input[i];
        let mut max = input[i];
        loop {
            j+=1;
            if input[j] > max {
                max = input[j];
            }
            if input[j] < min {
                min = input[j];
            }
            sum += input[j];
            if sum == v {
                return (min, max);
            }
            if sum > v {
                break;
            }
        }
    }
    return (0, 0);
}

pub fn run() {
    println!("day9");

    let mut input: Vec<i64> = Vec::new();

    if let Ok(lines) = utils::read_lines("data/day9.txt") {
        for line in lines {
            if let Ok(v) = line {
                if let Ok(n) = v.parse::<i64>() {
                    input.push(n);
                }
            }
        }
    }

    let v : i64 = part1(&input);
    println!("part1: {}", v);

    let result = part2(&input, v); 
    println!("part2: {}", result.0 + result.1);
}
