use super::utils;
use std::collections::HashMap;

fn count_group(answers: &Vec<String>) -> usize {
    let mut map: HashMap<char, bool> = HashMap::new();
    for ans in answers {
        for c in ans.chars() {
            map.insert(c, true);
        }
    }
    return map.len();
}

#[test]
fn test_count_group() {
    assert_eq!(3, count_group(&vec![String::from("abc")]));
    assert_eq!(
        3,
        count_group(&vec![
            String::from("a"),
            String::from("b"),
            String::from("c"),
        ])
    );
    assert_eq!(
        3,
        count_group(&vec![String::from("ab"), String::from("ac"),])
    );
}

fn count_group2(answers: &Vec<String>) -> usize {
    let mut map: HashMap<char, usize> = HashMap::new();
    for ans in answers {
        for c in ans.chars() {
            let count = map.entry(c).or_insert(0);
            *count += 1;
        }
    }
    let mut cnt: usize = 0;
    for (_, v) in map {
        if v == answers.len() {
            cnt += 1;
        }
    }
    return cnt;
}


#[test]
fn test_count_group2() {
    assert_eq!(3, count_group2(&vec![String::from("abc")]));
    assert_eq!(
        0,
        count_group2(&vec![
            String::from("a"),
            String::from("b"),
            String::from("c"),
        ])
    );
    assert_eq!(
        1,
        count_group2(&vec![String::from("ab"), String::from("ac"),])
    );
}

pub fn run() {
    println!("day6");

    let mut curr: Vec<String> = Vec::new();
    let mut sum: usize = 0;
    let mut sum2: usize = 0;
    if let Ok(lines) = utils::read_lines("data/day6.txt") {
        for line in lines {
            if let Ok(v) = line {
                if v.len() == 0 {
                    // empty line
                    sum += count_group(&curr);
                    sum2 += count_group2(&curr);
                    curr = Vec::new();
                    continue;
                }
                curr.push(v);
            }
        }
    }
    // last one
    sum += count_group(&curr);
    sum2 += count_group2(&curr);

    println!("part1: {}", sum);
    println!("part2: {}", sum2);
}
