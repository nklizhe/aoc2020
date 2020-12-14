use super::utils;
use std::collections::HashMap;
use std::mem;

#[derive(Clone, Debug, PartialEq)]
enum OpCode {
    Nop,
    Acc,
    Jmp,
}

#[derive(Clone, Debug)]
struct Op(OpCode, i32);

fn parse_line(line: &str) -> (OpCode, i32) {
    let tmp: Vec<&str> = line.split(" ").collect();
    let v: i32 = tmp[1].parse::<i32>().unwrap_or(0);
    match tmp[0] {
        "nop" => return (OpCode::Nop, 0),
        "acc" => return (OpCode::Acc, v),
        "jmp" => return (OpCode::Jmp, v),
        _ => return (OpCode::Nop, 0),
    };
}

#[test]
fn test_parse() {
    let v1 = parse_line("nop +0");
    assert_eq!(OpCode::Nop, v1.0);
    assert_eq!(0, v1.1);

    let v2 = parse_line("acc +1");
    assert_eq!(OpCode::Acc, v2.0);
    assert_eq!(1, v2.1);

    let v3 = parse_line("acc -3");
    assert_eq!(OpCode::Acc, v3.0);
    assert_eq!(-3, v3.1);
    let v4 = parse_line("jmp +4");
    assert_eq!(OpCode::Jmp, v4.0);
    assert_eq!(4, v4.1);

    let v5 = parse_line("jmp -6");
    assert_eq!(OpCode::Jmp, v5.0);
    assert_eq!(-6, v5.1);
}

fn execute(prog: &Vec<(OpCode, i32)>) -> (i32, i32) {
    let mut accu: i32 = 0;
    let mut counter: HashMap<i32, i32> = HashMap::new();
    let mut p: i32 = 0;
    let len = prog.len();
    loop {
        if p as usize >= len {
            break;
        }
        let c = counter.entry(p).or_insert(0);
        if *c >= 1 {
            break;
        }
        *c += 1;
        let curr = &prog[p as usize];
        match curr.0 {
            OpCode::Nop => {
                p += 1;
            }
            OpCode::Acc => {
                accu += curr.1;
                p += 1;
            }
            OpCode::Jmp => {
                p += curr.1;
            }
        }
    };
    return (p, accu);
}

fn part1(prog: &Vec<(OpCode, i32)>) -> i32 {
    let (_, accu) = execute(prog);
    return accu;
}

fn change_at(prog: &mut Vec<(OpCode, i32)>, i: usize) -> Vec<(OpCode, i32)> {
    let curr = &mut prog[i];
    match curr.0 {
        OpCode::Nop => curr.0 = OpCode::Jmp,
        OpCode::Jmp => curr.0 = OpCode::Nop,
        _ => {},
    }
    return prog.clone();
}

fn part2(prog: &mut Vec<(OpCode, i32)>) -> i32{
    let len = prog.len();
    for i in 0..prog.len() {
        let curr = &prog[i];
        match curr.0 {
            OpCode::Acc => continue,
            _ => {},
        }
        let v = change_at(prog, i);
        let (pos, accu) = execute(&v);
        if pos as usize == len {
            return accu;
        }
        change_at(prog, i);
    }
    return -1;
}

pub fn run() {
    println!("day8");

    let mut rules: Vec<(OpCode, i32)> = Vec::new();
    if let Ok(lines) = utils::read_lines("data/day8.txt") {
        for line in lines {
            if let Ok(v) = line {
                rules.push(parse_line(&v));
            }
        }
    }
    println!("part1: {}", part1(&rules));
    println!("part1: {}", part2(&mut rules));
}
