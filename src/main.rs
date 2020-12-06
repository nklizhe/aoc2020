use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

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

#[allow(dead_code)]
fn day1() {
    println!("day1");

    let mut input: Vec<i32> = Vec::new();

    if let Ok(lines) = read_lines("data/day1.txt") {
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

struct Policy {
    ch: char,
    min: i32,
    max: i32,
}

fn day2_part1(p: &Policy, pass: &str) -> bool {
    let mut n: i32 = 0;
    for c in pass.chars() {
        if c == p.ch {
            n = n + 1;
        }
    }
    return n >= p.min && n <= p.max;
}

fn day2_part2(p: &Policy, pass: &str) -> bool {
    let first: char = pass.chars().nth(p.min as usize - 1).unwrap_or('\0');
    let second: char = pass.chars().nth(p.max as usize - 1).unwrap_or('\0');

    return (first == p.ch || second == p.ch) && first != second;
}

#[allow(dead_code)]
fn day2() {
    println!("day2");

    let mut count1: i32 = 0;
    let mut count2: i32 = 0;
    if let Ok(lines) = read_lines("data/day2.txt") {
        for line in lines {
            if let Ok(v) = line {
                let tokens: Vec<&str> = v.split(" ").collect();
                let tmp: Vec<&str> = tokens[0].split("-").collect();
                let p: Policy = Policy {
                    ch: tokens[1].chars().nth(0).unwrap(),
                    min: tmp[0].parse::<i32>().unwrap(),
                    max: tmp[1].parse::<i32>().unwrap(),
                };

                if day2_part1(&p, tokens[2]) {
                    count1 = count1 + 1;
                }
                if day2_part2(&p, tokens[2]) {
                    count2 = count2 + 1;
                }
            }
        }
    }

    println!("part1: {}", count1);
    println!("part2: {}", count2);
}

#[test]
fn test_day2_part1() {
    assert_eq!(
        true,
        day2_part1(
            &Policy {
                ch: 'a',
                min: 1,
                max: 3
            },
            "abcde"
        )
    );
    assert_eq!(
        false,
        day2_part1(
            &Policy {
                ch: 'b',
                min: 1,
                max: 3
            },
            "cdefg"
        )
    );
    assert_eq!(
        true,
        day2_part1(
            &Policy {
                ch: 'c',
                min: 2,
                max: 9
            },
            "ccccccccc"
        )
    );
}

fn day3_calc(map: &Vec<String>, right: usize, down: usize) -> usize {
    let mut cnt: usize = 0;
    let mut n: usize = 0;
    for i in (0..map.len()).step_by(down) {
        if i == 0 {
            continue;
        }
        n = n + 1;
        let row: &String = &map[i];
        let p: usize = n * right;
        if row.chars().nth(p % row.len()).unwrap_or('\0') == '#' {
            cnt = cnt + 1;
        }
    }
    return cnt;
}

#[test]
fn test_day3_calc() {
    let map: Vec<String> = vec![
        String::from("..##......."),
        String::from("#...#...#.."),
        String::from(".#....#..#."),
        String::from("..#.#...#.#"),
        String::from(".#...##..#."),
        String::from("..#.##....."),
        String::from(".#.#.#....#"),
        String::from(".#........#"),
        String::from("#.##...#..."),
        String::from("#...##....#"),
        String::from(".#..#...#.#"),
    ];
    assert_eq!(2, day3_calc(&map, 1, 1));
    assert_eq!(7, day3_calc(&map, 3, 1));
    assert_eq!(3, day3_calc(&map, 5, 1));
    assert_eq!(4, day3_calc(&map, 7, 1));
    assert_eq!(2, day3_calc(&map, 1, 2));
}
fn day3_part1(map: &Vec<String>) -> usize {
    return day3_calc(map, 3, 1);
}

fn day3_part2(map: &Vec<String>) -> usize {
    let a1 = day3_calc(map, 1, 1);
    let a2 = day3_calc(map, 3, 1);
    let a3 = day3_calc(map, 5, 1);
    let a4 = day3_calc(map, 7, 1);
    let a5 = day3_calc(map, 1, 2);
    return a1 * a2 * a3 * a4 * a5;
}

fn day3() {
    println!("day3");

    let mut map: Vec<String> = Vec::new();
    if let Ok(lines) = read_lines("data/day3.txt") {
        for line in lines {
            if let Ok(v) = line {
                map.push(v);
            }
        }
    }
    println!("{}", day3_part1(&map));
    println!("{}", day3_part2(&map));
}

fn main() {
    // day1();
    //day2();
    day3();
}
