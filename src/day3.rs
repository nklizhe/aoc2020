use super::utils;

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

pub fn run() {
    println!("day3");

    let mut map: Vec<String> = Vec::new();
    if let Ok(lines) = utils::read_lines("data/day3.txt") {
        for line in lines {
            if let Ok(v) = line {
                map.push(v);
            }
        }
    }
    println!("{}", day3_part1(&map));
    println!("{}", day3_part2(&map));
}
