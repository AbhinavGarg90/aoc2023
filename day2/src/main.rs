use std::{collections::HashMap, fs, path::Path};

fn parseFunc(line: &str) -> u32 {
    let hm: HashMap<&str, u32> = HashMap::from([("green", 13), ("red", 12), ("blue", 14)]);
    let splitLine = line.split(":").collect::<Vec<&str>>();
    let mut gameNo = match (splitLine[0].split(" ").nth(1)) {
        Some(string) => string,
        None => return 0,
    }
    .parse::<u32>()
    .unwrap();
    splitLine[1].split(";").for_each(|game| {
        game.split(",").for_each(|pair| {
            let vals: Vec<&str> = pair
                .split(' ')
                .filter(|x| !x.is_empty())
                .collect::<Vec<&str>>();
            if *hm.get(vals[1]).unwrap() < vals[0].parse::<u32>().unwrap() {
                gameNo = 0;
            }
        })
    });
    return gameNo;
}

fn newParseFunc(line: &str) -> u32 {
    let mut hm: HashMap<&str, u32> = HashMap::from([("green", 0), ("red", 0), ("blue", 0)]);
    let splitLine = line.split(":").collect::<Vec<&str>>();
    let mut gameNo = match splitLine[0].split(" ").nth(1) {
        Some(string) => string,
        None => return 0,
    }
    .parse::<u32>()
    .unwrap();
    splitLine[1].split(";").for_each(|game| {
        game.split(",").for_each(|pair| {
            let vals: Vec<&str> = pair
                .split(' ')
                .filter(|x| !x.is_empty())
                .collect::<Vec<&str>>();
            hm.insert(
                vals[1],
                std::cmp::max(*hm.get(vals[1]).unwrap(), vals[0].parse::<u32>().unwrap()),
            );
        })
    });
    let a = hm.get("green").unwrap();
    dbg!(hm.drain().fold(1, |acc, val| { acc * (val.1) }))
}

fn main() {
    let filename = Path::new("/home/abhinavgarg/aoc2023/day2/src/input.txt");
    let file_contents = fs::read_to_string(filename).expect("failed to read file");
    let lines = file_contents.split("\n");
    let mut ret = 0;
    for line in lines {
        ret += newParseFunc(line);
    }
    println!("{ret}");
}
