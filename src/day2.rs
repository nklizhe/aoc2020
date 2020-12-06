use super::utils;

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

pub fn run() {
    println!("day2");

    let mut count1: i32 = 0;
    let mut count2: i32 = 0;
    if let Ok(lines) = utils::read_lines("data/day2.txt") {
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
