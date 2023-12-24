#[allow(dead_code)]
mod solve {
    use core::panic;
    use std::{cmp, collections::BTreeMap, fs, str::Lines};

    fn map_seed_to_loc(seed: i64, map_list: &[BTreeMap<(i64, i64), i64>; 7]) -> i64 {
        let mut ret: i64 = seed;
        for idx in 0..7 {
            ret = map_input(ret, &map_list[idx]);
        }
        ret
    }

    fn map_input(input: i64, map: &BTreeMap<(i64, i64), i64>) -> i64 {
        // if value within map ranges, return mapped, else return accumulated offset
        // offset is calculated by numbers of mapped targets ahead of input
        for ((lower, upper), offset) in map {
            if input >= *lower && input < *upper {
                return input + *offset;
            }
        }
        input
    }

    fn construct_map(mut lines: Lines<'_>) -> [BTreeMap<(i64, i64), i64>; 7] {
        // hashmap storage: <(low end of range, high end of range), offset>
        let mut hashmap_list: [BTreeMap<(i64, i64), i64>; 7] = Default::default();
        lines.nth(1);
        let mut idx: usize = 0;
        let mut line_values: [i64; 3] = [0, 0, 0];
        for line in lines {
            if line.contains(':') {
                idx += 1;
                continue;
            }
            if line.is_empty() {
                continue;
            }
            line.split(' ').enumerate().for_each(|(i, x)| {
                line_values[i] = x.parse().unwrap();
            });

            hashmap_list[idx].insert(
                (line_values[1], line_values[1] + line_values[2]),
                line_values[0] - line_values[1],
            );
        }
        hashmap_list
    }

    /// returns map with ((output_low, output_high), offset) ordered w.r.t output_low
    fn transform_map(map: &BTreeMap<(i64, i64), i64>) -> BTreeMap<(i64, i64), i64> {
        let mut ret: BTreeMap<(i64, i64), i64> = Default::default();
        for key in map.keys() {
            let value = map.get(key).unwrap();
            ret.insert((key.0 + value, key.1 + value), -value);
        }
        ret
    }

    fn if_val_in_range(vec: &Vec<(i64, i64)>, target: i64) -> Option<i64> {
        for val in vec.iter() {
            if target >= val.0 && target < val.1 {
                return Some(target);
            }
        }
        None
    }
    fn smallest_common_list(
        list_of_ranges: &Vec<(i64, i64)>,
        range2: (i64, i64),
    ) -> Option<(i64, i64)> {
        let mut lowest = smallest_common(list_of_ranges[0], range2);
        for range1 in list_of_ranges.iter() {
            match lowest {
                Some(value1) => match smallest_common(*range1, range2) {
                    Some(value2) => {
                        if value2.0 < value1.0 {
                            lowest = Some(value2)
                        }
                    }
                    None => {}
                },
                None => {}
            }
        }
        lowest
    }

    fn smallest_common(range1: (i64, i64), range2: (i64, i64)) -> Option<(i64, i64)> {
        match (range1, range2) {
            ((lower1, upper1), (lower2, upper2)) if lower1 < upper2 && upper1 == -1 => {
                Some((cmp::max(lower1, lower2), upper2))
            }
            ((lower1, upper1), (lower2, upper2)) if lower2 < upper1 && upper2 == -1 => {
                Some((cmp::max(lower1, lower2), upper1))
            }
            ((lower1, upper1), (lower2, upper2)) if (lower1 <= lower2 && upper1 > lower2) => {
                Some((lower2, cmp::min(upper1, upper2)))
            }
            ((lower1, upper1), (lower2, upper2)) if lower1 >= lower2 && lower1 < upper2 => {
                Some((lower1, cmp::min(upper1, upper2)))
            }
            ((lower1, upper1), (lower2, upper2)) if lower1 == lower2 => Some((lower1, lower1)),
            ((lower1, _upper1), (_lower2, upper2)) if lower1 >= upper2 => {
                // range1 above range2
                None
            }
            ((_lower1, upper1), (lower2, _upper2)) if upper1 <= lower2 => {
                // range 1 completely below range 2
                None
            }
            _ => panic!("smallest_common unexpected values"),
        }
    }

    /// used to setup and make recursive call; this functions does minimization, recursive call function just looks for
    /// possible matches using recursion
    fn find_min(
        seeds_vec: Vec<(i64, i64)>,
        b_tree_map_list: &[BTreeMap<(i64, i64), i64>; 7],
    ) -> (i64, i64) {
        let mut prev_max: i64 = 0;
        for key in transform_map(&b_tree_map_list[6]) {
            match find_min_recursive(&seeds_vec, b_tree_map_list, 5, (prev_max, key.0 .0)) {
                Some(value) => {
                    return value;
                }
                None => {}
            }
            match find_min_recursive(
                &seeds_vec,
                b_tree_map_list,
                5,
                (key.0 .0 + key.1, key.0 .1 + key.1),
            ) {
                Some(value) => {
                    return value;
                }
                None => {}
            }
            prev_max = key.0 .1;
        }
        (prev_max, -1)
    }
    fn find_min_recursive(
        seeds_vec: &Vec<(i64, i64)>,
        b_tree_map_list: &[BTreeMap<(i64, i64), i64>; 7],
        idx: isize,
        target: (i64, i64),
    ) -> Option<(i64, i64)> {
        // by looking at maps, figure out how to arrive at the minimum value
        // smallest value can either be 1:1 from input, or based on an offset
        // based on which one is lower, try to find a path that allows for that to be
        // reached by reverse-mapping
        // if not found, backtrack once, and try less effecient path

        // base case idx=0 or impossible value
        // if idx = 0, check if seed exists. if not return None, and look for other matches
        if idx == -1 {
            let ret = smallest_common_list(seeds_vec, target);
            return ret;
        }
        // if idx == -1 {
        //     return Some(target);
        // }
        let transformed_map = transform_map(&b_tree_map_list[idx as usize]);

        // recursive case: find and order how to arrive at smallest value
        let mut prev_max: i64 = 0;
        for val in transformed_map {
            match smallest_common((prev_max, val.0 .0), target) {
                Some((lower, upper)) => {
                    // println!("at idx {} found {:?} in {:?}", idx, (lower, upper), target);
                    match find_min_recursive(seeds_vec, b_tree_map_list, idx - 1, (lower, upper)) {
                        Some(val) => return Some(val),
                        None => {}
                    }
                }
                None => {
                    // println!("failed to match \n");
                }
            }
            match smallest_common(val.0, target) {
                Some((lower, upper)) => {
                    // println!(
                    //     "at idx {} mapping by {} found for {:?} in {:?}",
                    //     idx,
                    //     val.1,
                    //     (lower, upper),
                    //     target
                    // );
                    match find_min_recursive(
                        seeds_vec,
                        b_tree_map_list,
                        idx - 1,
                        (lower + val.1, upper + val.1),
                    ) {
                        Some(val) => return Some(val),
                        None => {}
                    }
                }

                None => {
                    // println!("failed to match \n");
                }
            }
            prev_max = val.0 .1 + val.1;
        }
        match smallest_common((prev_max, -1), target) {
            Some((lower, upper)) => {
                // println!(
                //     "at idx {} found at end range for {:?} in {:?}",
                //     idx,
                //     (lower, upper),
                //     target
                // );
                match find_min_recursive(seeds_vec, b_tree_map_list, idx - 1, (lower, upper)) {
                    Some(val) => return Some(val),
                    None => {}
                }
            }

            None => {
                // println!("failed to match \n");
            }
        }
        None
    }

    fn create_seed_vec(lines: &mut Lines<'_>) -> Vec<i64> {
        lines
            .next()
            .unwrap()
            .strip_prefix("seeds: ")
            .unwrap()
            .split(' ')
            .map(|x| x.parse::<i64>().unwrap())
            .collect::<Vec<i64>>()
    }

    fn parse_file(input_file: String) -> i64 {
        let file_content = fs::read_to_string(input_file).unwrap();
        let mut lines = file_content.lines();
        // get seeds
        let seeds_vec = create_seed_vec(&mut lines);
        let btree_map_list = construct_map(lines);
        let mut ret: i64 = map_seed_to_loc(seeds_vec[0], &btree_map_list);
        let mut iter_val: i64;

        for (_idx, seed) in seeds_vec.iter().enumerate() {
            iter_val = map_seed_to_loc(*seed, &btree_map_list);
            if iter_val < ret {
                ret = iter_val;
            }
        }
        ret
    }

    fn parse_file_pt2(input_file: String) -> i64 {
        let file_content = fs::read_to_string(input_file).unwrap();
        let mut lines = file_content.lines();
        // get seeds
        let mut seeds_vec: Vec<(i64, i64)> = Vec::new();
        lines
            .next()
            .unwrap()
            .strip_prefix("seeds: ")
            .unwrap()
            .split(' ')
            .fold(0, |acc, val| {
                let seed: i64 = val.parse().unwrap();
                match acc % 2 {
                    0 => {
                        seeds_vec.push((seed, seed));
                        return acc + 1;
                    }
                    1 => {
                        seeds_vec[acc / 2].1 += seed;
                    }
                    _ => {}
                }
                acc + 1
            });
        let b_tree_map_list = construct_map(lines);
        let ans_range = find_min(seeds_vec, &b_tree_map_list);
        let mut min: i64 = map_seed_to_loc(ans_range.0, &b_tree_map_list);
        for i in ans_range.0..ans_range.1 {
            let iter_val = map_seed_to_loc(i, &b_tree_map_list);
            if iter_val < min {
                min = iter_val;
            }
        }
        min
    }

    #[test]
    fn part_1_init() {
        let input_file = "/home/abhinavgarg/aoc2023/day5/src/input.txt".to_string();
        assert_eq!(parse_file(input_file.clone()), 35);
    }

    #[test]
    fn part_1() {
        let input_file = "/home/abhinavgarg/aoc2023/day5/src/test.txt".to_string();
        // println!("\n\n\n\n");
        assert_eq!(parse_file(input_file), 261668924);
    }

    #[test]
    fn part_2_init() {
        let input_file = "/home/abhinavgarg/aoc2023/day5/src/input.txt".to_string();
        assert_eq!(parse_file_pt2(input_file), 46);
    }

    #[test]
    fn part_2() {
        let input_file = "/home/abhinavgarg/aoc2023/day5/src/test.txt".to_string();
        dbg!(parse_file_pt2(input_file));
        assert!(1 == 0);
    }
}

fn main() {
    println!("Hello, world!");
}
