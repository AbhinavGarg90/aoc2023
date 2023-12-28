/*
Time:      7  15   30
Distance:  9  40  200

Don't hold the button at all (that is, hold it for 0 milliseconds) at the start of the race. The boat won't move; it will
have traveled 0 millimeters by the end of the race.
Hold the button for 1 millisecond at the start of the race. Then, the boat will travel at a speed of 1
 millimeter per millisecond for 6 milliseconds, reaching a total distance traveled of 6 millimeters.
Hold the button for 2 milliseconds, giving the boat a speed of 2 millimeters per millisecond. It will then get 5
 milliseconds to move, reaching a total distance of 10 millimeters.
Hold the button for 3 milliseconds. After its remaining 4 milliseconds of travel time, the boat will have gone 12 millimeters.
Hold the button for 4 milliseconds. After its remaining 3 milliseconds of
travel time, the boat will have gone 12 millimeters.
Hold the button for 5 milliseconds, causing the boat to travel a
total of 10 millimeters.
Hold the button for 6 milliseconds, causing the boat to travel a
total of 6 millimeters.
Hold the button for 7 milliseconds. That's the entire duration
of the race. You never let go of the button. The boat can't move until you let go of the button. Please
make sure you let go of the button so the boat gets to move. 0 millimeters.

*/

use std::fs;

fn solve_pt1(input_file: String) -> i64 {
    let file_text = fs::read_to_string(input_file).unwrap();
    let lines = file_text.lines();

    // convert data into two vectors
    let mut data_vecs: [Vec<i64>; 2] = Default::default();
    lines.enumerate().for_each(|(i, val)| {
        data_vecs[i] = val
            .split(": ")
            .nth(1)
            .unwrap()
            .split(" ")
            .filter(|x| *x != "")
            .map(|x| x.parse::<i64>().unwrap())
            .collect::<Vec<i64>>();
    });
    let race_count = data_vecs[0].len();
    let mut ret = 1;
    for i in 0..race_count {
        ret *= race_possibilites(data_vecs[0][i], data_vecs[1][i]);
    }

    ret
}

fn solve_pt2(input_file: String) -> i64 {
    let file_text = fs::read_to_string(input_file).unwrap();
    let lines = file_text.lines();

    // convert data into two vectors
    let mut data_vecs: [i64; 2] = Default::default();
    lines.enumerate().for_each(|(i, val)| {
        data_vecs[i] = val
            .split(": ")
            .nth(1)
            .unwrap()
            .split(" ")
            .filter(|x| *x != "")
            .fold("".to_string(), |mut acc, x| {
                acc.push_str(x);
                acc
            })
            .parse()
            .unwrap()
    });
    race_possibilites(data_vecs[0], data_vecs[1])
}
/*
 distance = v*t
 distance = (T-t)*t
 dx/dt = T - 2*t
    condition that needs to be met
 max < (T-t)*t
 0 < (T-t)*t - max
 find integer values where RHS greater than 0
 eqn: 0 = -t**2 +T*t - max
*/

fn race_possibilites(time: i64, distance: i64) -> i64 {
    let float_time = time as f64;
    let float_distance = distance as f64;
    let det = (float_time * float_time + ((4.0f64) * (-float_distance))).sqrt();
    let root1 = (-float_time + det) / (-2.0f64);
    let root2 = (-float_time - det) / (-2.0f64);
    let mut ret: i64 = (root2.floor() - root1.ceil()) as i64 + 1;

    if root1 == root1.ceil() {
        ret -= 1;
    }

    if root2 == root2.floor() {
        ret -= 1;
    }
    ret
}

fn main() {
    println!("Hello, world!");
}

#[test]
fn solve_pt1_test_init() {
    let input_file = "/home/abhinavgarg/aoc2023/day6/src/input.txt".to_string();
    assert_eq!(solve_pt1(input_file), 288);
}

#[test]
fn solve_pt1_final() {
    let input_file = "/home/abhinavgarg/aoc2023/day6/src/test.txt".to_string();
    dbg!(solve_pt1(input_file));
}

#[test]
fn solve_pt2_init() {
    let input_file = "/home/abhinavgarg/aoc2023/day6/src/input.txt".to_string();
    assert_eq!(solve_pt2(input_file), 71503);
}

#[test]
fn solve_pt2_final() {
    let input_file = "/home/abhinavgarg/aoc2023/day6/src/test.txt".to_string();
    assert_eq!(solve_pt2(input_file), 34655848);
}
