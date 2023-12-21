use core::panic;
use std::{
    ascii::AsciiExt,
    collections::{btree_map::Values, HashSet},
    fs,
};

fn parse_input_string(line: String) -> i32 {
    let vals = line.split(": ").collect::<Vec<&str>>()[1]
        .split(" | ")
        .collect::<Vec<&str>>();
    let hash_set: HashSet<i32> = HashSet::from_iter(
        vals[0]
            .split(' ')
            .filter(|x| *x != "")
            .map(|x| x.parse::<i32>().unwrap()),
    );
    let ret_vec = Vec::from_iter(
        vals[1]
            .split(' ')
            .filter(|x| *x != "")
            .map(|x| x.parse::<i32>().unwrap())
            .filter(|x| hash_set.contains(x)),
    );
    ret_vec.len() as i32
}

fn evaluate_count(n: i32) -> i32 {
    if n == 0 {
        return 0;
    }
    (2 as i32).pow((n - 1) as u32)
}

fn solve(input_file: String) -> i32 {
    let input_text = fs::read_to_string(input_file).unwrap();
    let lines = input_text.lines();
    lines.fold(0, |acc, x| {
        acc + evaluate_count(parse_input_string(x.to_string()))
    })
}

fn solve_pt2(input_file: String) -> i32 {
    let input_text = fs::read_to_string(input_file).unwrap();
    let lines = input_text.lines();
    let mut count_vec: Vec<i32> = Vec::new();
    let card_vals: Vec<i32> = Vec::from_iter(lines.map(|x| {
        count_vec.push(1);
        parse_input_string(x.to_string())
    }));
    for (idx, val) in card_vals.iter().enumerate() {
        dbg!(&count_vec);
        if *val == 0 {
            continue;
        }
        for offset in 1..=*val {
            count_vec[idx + offset as usize] += count_vec[idx];
        }
    }
    count_vec.iter().fold(0, |acc, x| acc + x)
}

fn main() {
    let input_file = "/home/abhinavgarg/aoc2023/day4/test.txt";
    println!("{}", solve_pt2(input_file.to_owned()));
}

#[test]
fn pt1_init() {
    let input_file = "/home/abhinavgarg/aoc2023/day4/input.txt";
    assert_eq!(solve(input_file.to_owned()), 13);
}
#[test]
fn pt1() {
    let input_file = "/home/abhinavgarg/aoc2023/day4/test.txt";
    assert_eq!(solve(input_file.to_owned()), 23028);
}

#[test]
fn pt2() {
    let input_file = "/home/abhinavgarg/aoc2023/day4/input.txt";
    assert_eq!(solve_pt2(input_file.to_string()), 30);
}
