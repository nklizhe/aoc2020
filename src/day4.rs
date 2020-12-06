use super::utils;
use std::collections::HashMap;
use std::i64;

type Passport = HashMap<String, String>;

fn is_valid_ecl(ecl: &str) -> bool {
    match ecl {
        "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth" => return true,
        _ => return false,
    }
}

fn is_valid_byr(byr: &str) -> bool {
    if byr.len() != 4 {
        return false;
    }
    let byr_value = byr.parse::<i32>().unwrap_or(-1);
    return byr_value >= 1920 && byr_value <= 2002;
}

fn is_valid_iyr(iyr: &str) -> bool {
    if iyr.len() != 4 {
        return false;
    }
    let iyr_value = iyr.parse::<i32>().unwrap_or(-1);
    return iyr_value >= 2010 && iyr_value <= 2020;
}

fn is_valid_eyr(eyr: &str) -> bool {
    if eyr.len() != 4 {
        return false;
    }
    let eyr_value = eyr.parse::<i32>().unwrap_or(-1);
    return eyr_value >= 2020 && eyr_value <= 2030;
}

fn is_valid_hgt(hgt: &str) -> bool {
    let hgt_unit = &hgt[hgt.len() - 2..];
    let hgt_value = hgt[..hgt.len() - 2].parse::<i32>().unwrap_or(-1);
    match hgt_unit {
        "cm" => hgt_value >= 150 && hgt_value <= 193,
        "in" => hgt_value >= 59 && hgt_value <= 76,
        _ => false,
    }
}

fn is_valid_hcl(hcl: &str) -> bool {
    if hcl.len() != 7 {
        return false;
    }
    let hcl_value = i64::from_str_radix(&hcl[1..], 16);
    return hcl_value.is_ok();
}

fn is_valid_pid(pid: &str) -> bool {
    if pid.len() != 9 {
        return false;
    }
    for c in pid.chars() {
        if c < '0' || c > '9' {
            return false;
        }
    }
    return true;
}

fn validate(p: &Passport) -> bool {
    let keys = vec!["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];
    for k in keys {
        if !p.contains_key(k) {
            return false;
        }
    }
    return true;
}

fn validate2(p: &Passport) -> bool {
    if !validate(p) {
        return false;
    }
    println!("password {:?}", p);
    if !is_valid_byr(&p["byr"]) {
        println!("invalid byr");
        return false;
    }
    if !is_valid_iyr(&p["iyr"]) {
        println!("invalid iyr");
        return false;
    }
    if !is_valid_eyr(&p["eyr"]) {
        println!("invalid eyr");
        return false;
    }
    if !is_valid_hgt(&p["hgt"]) {
        println!("invalid hgt");
        return false;
    }
    if !is_valid_hcl(&p["hcl"]) {
        println!("invalid hcl");
        return false;
    }
    if !is_valid_ecl(&p["ecl"]) {
        println!("invalid ecl");
        return false;
    }
    if !is_valid_pid(&p["pid"]) {
        println!("invalid pid");
        return false;
    }
    return true;
}

fn do_run(input: &Vec<Passport>, validator: fn(&Passport) -> bool) -> usize {
    let mut cnt: usize = 0;
    for p in input {
        if validator(&p) {
            cnt = cnt + 1;
        }
    }
    return cnt;
}

pub fn run() {
    println!("day4");

    let mut input: Vec<Passport> = Vec::new();
    let mut curr: Passport = Passport::new();
    if let Ok(lines) = utils::read_lines("data/day4.txt") {
        for line in lines {
            if let Ok(v) = line {
                if v.len() == 0 {
                    // empty line
                    input.push(curr);
                    curr = Passport::new();
                    continue;
                }
                let fields: Vec<&str> = v.split(" ").collect();
                for f in fields {
                    let kv: Vec<&str> = f.split(":").collect();
                    curr.insert(String::from(kv[0]), String::from(kv[1]));
                }
            }
        }
    }
    input.push(curr);
    println!("part1: {}", do_run(&input, validate));
    println!("part2: {}", do_run(&input, validate2));
}
