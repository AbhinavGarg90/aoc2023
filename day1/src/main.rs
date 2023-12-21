use std::{fs::{self, File}, io::Write};



fn main() {

    let filename = "input.txt";
    let file_contents = fs::read_to_string(filename).expect("Error in reading file");
    let lines = file_contents.split("\n");
    let mut running_sum: u32 = 0;
    for line in lines {
        let nums: Vec<u32> = line
                .replace("one", "on1e")
                .replace("two", "t2wo")
                .replace("three", "thr3ee")
                .replace("four", "fo4ur")
                .replace("five", "fi5ve")
                .replace("six", "si6x")
                .replace("seven", "sev7en")
                .replace("eight", "eig8ht")
                .replace("nine", "ni9ne")
                .chars()
                .filter(|character| ((*character as i32) > 0x30 && (*character as i32) < 0x3A))
                .map(|char| char.to_digit(10).unwrap())
                .collect();
        running_sum += (nums[0])*10 + nums[nums.len()-1];; 

    }
    let mut file = File::create("output.txt").unwrap();
    file.write_fmt(format_args!("{}", running_sum)).unwrap();

}
