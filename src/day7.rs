use super::utils;
use std::collections::HashMap;

type BagList = HashMap<String, usize>;
type BagRules = HashMap<String, BagList>;

fn contains(rules: &BagRules, key: &str) -> bool {
    if key == "shiny gold bags" {
        return false;
    }
    if !rules.contains_key(key) {
        return false;
    }
    let r: &BagList = rules.get(key).unwrap();
    if r.contains_key("shiny gold bags") {
        return true;
    }
    for (k, v) in r {
        if *v > 0 {
            if contains(rules, k) {
                return true;
            }
        }
    }
    return false;
}

fn bags(rules: &BagRules, key: &str) -> usize {
    if !rules.contains_key(key) {
        return 0;
    }
    let r: &BagList = rules.get(key).unwrap();
    let mut cnt: usize = 1;
    for (k, v) in r {
        if *v > 0 {
            cnt += (*v) * bags(rules, k);
        }
    }
    return cnt;
}
fn parse(line: &str) -> (String, BagList) {
    let mut list: BagList = BagList::new();
    let tmp: Vec<&str> = line[..line.len() - 1].split(" contain ").collect();
    let r: String = String::from(tmp[1]);
    if r == "no other bags" {
        return (String::from(tmp[0]), list);
    }
    // split by ", "
    let tmp2: Vec<&str> = r.split(", ").collect();
    for x in tmp2 {
        // find the first ' '
        let mut i = 0;
        for c in x.chars() {
            i += 1;
            if c == ' ' {
                break;
            }
        }
        let val = x[..i - 1].parse::<usize>().unwrap(); // number of bags
        let mut key: String = String::from(&x[i..]);
        // append 's' to '... bag'
        if val == 1 {
            key.push('s');
        }
        list.insert(key, val);
    }
    return (String::from(tmp[0]), list);
}

#[test]
fn test_parse() {
    let (k, v) = parse("light red bags contain 1 bright white bag, 2 muted yellow bags.");
    assert_eq!("light red bags", k);
    assert_eq!(&1, v.get("bright white bags").unwrap());
    assert_eq!(&2, v.get("muted yellow bags").unwrap());
}

fn part1(rules: &BagRules) -> usize {
    let mut cnt: usize = 0;
    for k in rules.keys() {
        if contains(rules, k) {
            cnt += 1;
        }
    }
    return cnt;
}

fn part2(rules: &BagRules) -> usize {
    return bags(rules, "shiny gold bags") - 1;
}

pub fn run() {
    println!("day7");

    let mut rules: BagRules = BagRules::new();
    if let Ok(lines) = utils::read_lines("data/day7.txt") {
        for line in lines {
            if let Ok(v) = line {
                let (k, v) = parse(&v);
                rules.insert(k, v);
            }
        }
    }
    println!("{:?}, {}", rules, rules.len());
    println!("part1: {}", part1(&rules));
    println!("part2: {}", part2(&rules));
}
