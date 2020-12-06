use super::utils;
use std::collections::HashMap;

fn find_range(start: u32, end: u32, front: bool) -> (u32, u32) {
    if front {
        return (start, start + (end - start) / 2);
    } else {
        return (start + (end - start) / 2 + 1, end);
    }
}

#[test]
fn test_find_range() {
    assert_eq!((0, 63), find_range(0, 127, true));
    assert_eq!((64, 127), find_range(0, 127, false));
    assert_eq!((33, 33), find_range(33, 33, true));
}

fn find_row(codes: &str) -> u32 {
    let mut start = 0;
    let mut end = 127;
    let mut x: (u32, u32);
    for c in codes.chars() {
        match c {
            'F' => x = find_range(start, end, true),
            'B' => x = find_range(start, end, false),
            _ => continue,
        };
        start = x.0;
        end = x.1;
    }
    return start;
}

#[test]
fn test_find_row() {
    assert_eq!(70, find_row("BFFFBBFRRR"));
    assert_eq!(14, find_row("FFFBBBFRRR"));
    assert_eq!(102, find_row("BBFFBBFRLL"));
}

fn find_col(codes: &str) -> u32 {
    let mut start = 0;
    let mut end = 7;
    let mut x: (u32, u32);
    for c in codes.chars() {
        match c {
            'L' => x = find_range(start, end, true),
            'R' => x = find_range(start, end, false),
            _ => continue,
        };
        start = x.0;
        end = x.1;
    }
    return start;
}

#[test]
fn test_find_col() {
    assert_eq!(7, find_col("BFFFBBFRRR"));
    assert_eq!(7, find_col("FFFBBBFRRR"));
    assert_eq!(4, find_col("BBFFBBFRLL"));
}

fn get_seat_id(row: u32, col: u32) -> u32 {
    return row * 8 + col;
}

pub fn run() {
    println!("day5");

    let mut list: HashMap<u32, bool> = HashMap::new();
    let mut lowest = get_seat_id(128, 7);
    let mut highest = 0;
    if let Ok(lines) = utils::read_lines("data/day5.txt") {
        for line in lines {
            if let Ok(v) = line {
                let row = find_row(&v);
                let col = find_col(&v);
                let seat_id = get_seat_id(row, col);
                if seat_id > highest {
                    highest = seat_id;
                }
                if seat_id < lowest {
                    lowest = seat_id;
                }
                list.insert(seat_id, true);
            }
        }
    }

    println!("day5 part1: {}", highest);

    for i in lowest + 1..highest {
        if !list.contains_key(&i) {
            println!("day5 part2: {}", i);
            break;
        }
    }
}
