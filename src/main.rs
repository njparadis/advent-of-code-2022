use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let mut elves: Vec<u64> = Vec::new();
    // File must exist in current path before this produces output
    if let Ok(lines) = read_lines("./inputs/day_1_1.txt") {
        let mut current_elf: u64 = 0;
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(calorie_value) = line {
                if calorie_value.eq("") {
                    elves.push(current_elf);
                    current_elf = 0;
                } else {
                    current_elf += calorie_value.parse::<u64>().unwrap();
                }
            }
        }
    }
    elves.sort();
    elves.reverse();
    println!("{}", elves[0] + elves[1] + elves[2]);
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
