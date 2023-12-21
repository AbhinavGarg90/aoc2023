use core::{fmt, num};
use std::{collections::HashSet, default, fs, future::IntoFuture};
#[derive(Clone, Copy, Debug)]
struct Point {
    row: u32,
    col: u32,
}

#[derive(Debug)]
struct Matrix {
    vec: Vec<u8>,
    width: u32,
    height: u32,
}

impl Point {
    fn try_new(row: i32, col: i32) -> Option<Point> {
        if row < 0 || col < 0 {
            return None;
        }
        Some(Point {
            row: (row as u32),
            col: (col as u32),
        })
    }
    fn isValid(&self, matrix: &Matrix) -> Option<Point> {
        if self.row < matrix.height && self.col < matrix.width {
            return Some(*self);
        }
        None
    }

    fn new_and_valid(row: i32, col: i32, matrix: &Matrix) -> Option<Point> {
        let pt = Point::try_new(row, col);

        match pt {
            Some(pt) => pt.isValid(&matrix),
            None => None,
        }
    }

    fn index(&self, matrix: &Matrix) -> usize {
        (self.row * matrix.width + self.col) as usize
    }
}

impl Matrix {
    fn new(input_vec: Vec<u8>) -> Matrix {
        let mut width: u32 = 0;
        for (i, val) in input_vec.iter().enumerate() {
            if *val == 10 {
                width = i as u32;
                break;
            }
        }
        let mut new_vec: Vec<u8> = Vec::new();
        for val in input_vec.iter() {
            if *val == 10 {
                continue;
            }
            new_vec.push(*val);
        }
        let height = input_vec.len() as u32 / width;
        Matrix {
            vec: new_vec,
            width: width,
            height: height,
        }
    }
    fn adjacent_pts(&self, point: Point) -> [Option<Point>; 8] {
        let pt_idx = self.width * point.row + point.col;
        let row = point.row as i32;
        let col = point.col as i32;
        [
            Point::new_and_valid(row, col - 1, self),
            Point::new_and_valid(row, col + 1, self),
            Point::new_and_valid(row - 1, col + 1, self),
            Point::new_and_valid(row + 1, col + 1, self),
            Point::new_and_valid(row + 1, col, self),
            Point::new_and_valid(row - 1, col, self),
            Point::new_and_valid(row + 1, col - 1, self),
            Point::new_and_valid(row - 1, col - 1, self),
        ]
    }
    fn adjacent_to_symbol(&self, point: Point) -> bool {
        let pts = self.adjacent_pts(point);
        for pt in pts {
            match pt {
                Some(x) => {
                    let val = self.vec[(x.col + x.row * self.width) as usize];
                    if ((val > 32 && val < 48) || (val > 57 && val < 65)) && val != 46 {
                        return true;
                    }
                }
                None => (),
            }
        }
        false
    }

    // given a point on a matrix, returns a hashset containing rightmost indices of the surrounding numbers
    fn adjacent_numbers(&self, point: Point) -> HashSet<(u32, usize)> {
        let points = self.adjacent_pts(point);
        let mut ret_hash = HashSet::new();
        for pt in points {
            match pt {
                Some(x) => {
                    if (self.vec[x.index(self)] as char).is_ascii_digit() {
                        ret_hash.insert(evaluate_value_non_mut(self, x));
                    }
                }
                None => {}
            }
        }
        ret_hash
    }
}

// finds the value of continuous string of numbers, mutates the values to null and returns the total value, along with the start index of the numeber
fn evaluate_value(matrix: &mut Matrix, point: Point) -> (u32, usize) {
    let row = point.row;
    let col = point.col;
    // find the whole value of the number, extract and set to 0
    let mut right_index: usize = (row * matrix.width + col) as usize;
    // remain on the same row, and find last numerical in string
    while right_index + 1 < (matrix.width * (row + 1)) as usize
        && (matrix.vec[right_index + 1] as char).is_ascii_digit()
    {
        right_index += 1;
    }
    // calculate total value going left to right
    let mut total: u32 = 0;
    let mut scalar: u32 = 1;
    while (matrix.vec[right_index as usize] as char).is_numeric()
        && right_index >= (row * matrix.width) as usize
    {
        // index points to number char, scale, convert and add
        total += scalar * (matrix.vec[right_index as usize] - 48) as u32;
        scalar *= 10;
        matrix.vec[right_index as usize] = 0;
        if right_index > 0 {
            right_index -= 1;
        } else {
            break;
        }
    }
    (total, right_index)
}

// finds the value of continuous string of numbers, mutates the values to null and returns the total value, along with the start index of the numeber
fn evaluate_value_non_mut(matrix: &Matrix, point: Point) -> (u32, usize) {
    let row = point.row;
    let col = point.col;
    // find the whole value of the number, extract and set to 0
    let mut right_index: usize = (row * matrix.width + col) as usize;
    // remain on the same row, and find last numerical in string
    while right_index + 1 < (matrix.width * (row + 1)) as usize
        && (matrix.vec[right_index + 1] as char).is_ascii_digit()
    {
        right_index += 1;
    }
    // calculate total value going left to right
    let mut total: u32 = 0;
    let mut scalar: u32 = 1;
    while (matrix.vec[right_index as usize] as char).is_ascii_digit()
        && right_index >= (row * matrix.width) as usize
    {
        // index points to number char, scale, convert and add
        total += scalar * (matrix.vec[right_index as usize] - 48) as u32;
        scalar *= 10;
        if right_index > 0 {
            right_index -= 1;
        } else {
            break;
        }
    }
    (total, right_index)
}

fn parse(input_file: String) -> u32 {
    let mut ret: u32 = 0;
    let file_contents = fs::read(input_file).unwrap();
    let mut matrix = Matrix::new(file_contents);
    for row in 0..matrix.height {
        for col in 0..matrix.width {
            let point = Point { row, col };
            if (matrix.adjacent_to_symbol(point)
                && (matrix.vec[(row * matrix.width + col) as usize] as char).is_ascii_digit())
            {
                ret += evaluate_value(&mut matrix, point).0;
            }
        }
    }
    ret
}

fn parse_pt2(input_file: String) -> u32 {
    let mut ret = 0;
    let file_contents = fs::read(input_file).unwrap();
    let mut matrix = Matrix::new(file_contents);
    for row in 0..matrix.height {
        for col in 0..matrix.width {
            let point = Point { row, col };
            // given value is a gear, carry out further operations
            if (matrix.vec[point.index(&matrix)] == 42) {
                let mut numbers = matrix.adjacent_numbers(point);
                if numbers.len() == 2 {
                    ret += numbers.drain().fold(1, |acc, x| acc * x.0);
                }
            }
        }
    }
    return ret;
}

fn main() {
    let file_path: String = "/home/abhinavgarg/aoc2023/day3/src/input.txt".to_string();
    println!("{}", parse_pt2(file_path));
}
#[test]
fn test_part1_init() {
    let input_file = "/home/abhinavgarg/aoc2023/day3/src/test.txt";
    assert_eq!(parse(input_file.to_owned()), 4361);
}

#[test]
fn test_part1() {
    let input_file = "/home/abhinavgarg/aoc2023/day3/src/input.txt";
    assert_eq!(parse(input_file.to_owned()), 512794);
}

#[test]
fn test_part2_init() {
    let input_file = "/home/abhinavgarg/aoc2023/day3/src/test.txt";
    assert_eq!(parse_pt2(input_file.to_owned()), 467835);
}
