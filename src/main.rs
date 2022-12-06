use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashSet;

fn main() {
    day_1();
    day_2();
    day_3();
    day_4();
    day_5();
    day_6();
}

fn day_1() {
    println!("Day 1");
    let mut elves: Vec<u64> = Vec::new();
    // File must exist in current path before this produces output
    if let Ok(lines) = read_lines("./inputs/day_1.txt") {
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
    println!("Part 1: {}", elves.get(0).unwrap_or(&0u64));
    println!(
        "Part 2: {}",
        elves.get(0).unwrap_or(&0u64)
            + elves.get(1).unwrap_or(&0u64)
            + elves.get(2).unwrap_or(&0u64)
    );
}

fn day_2() {
    println!("Day 2");
    let mut score_1: u64 = 0;
    if let Ok(lines) = read_lines("./inputs/day_2.txt") {
        for line in lines {
            if let Ok(round) = line {
                let plays: Vec<&str> = round.split_whitespace().collect();
                let opp = plays.get(0).unwrap_or(&"");
                let me = plays.get(1).unwrap_or(&"");
                score_1 += match me {
                    &"X" => 1,
                    &"Y" => 2,
                    &"Z" => 3,
                    _ => 0,
                };
                if (me.eq(&"X") && opp.eq(&"A"))
                    || (me.eq(&"Y") && opp.eq(&"B"))
                    || (me.eq(&"Z") && opp.eq(&"C"))
                {
                    score_1 += 3;
                }
                if (me.eq(&"X") && opp.eq(&"C"))
                    || (me.eq(&"Y") && opp.eq(&"A"))
                    || (me.eq(&"Z") && opp.eq(&"B"))
                {
                    score_1 += 6;
                }
            }
        }
    }
    println!("Part 1: {}", score_1);

    let mut score_2: u64 = 0;
    if let Ok(lines) = read_lines("./inputs/day_2.txt") {
        for line in lines {
            if let Ok(round) = line {
                let plays: Vec<&str> = round.split_whitespace().collect();
                let opp = plays.get(0).unwrap_or(&"");
                let me = plays.get(1).unwrap_or(&"");
                score_2 += match me {
                    &"X" => 0,
                    &"Y" => 3,
                    &"Z" => 6,
                    _ => unreachable!(),
                };
                if me.eq(&"X") {
                    score_2 += match opp {
                        &"A" => 3,
                        &"B" => 1,
                        &"C" => 2,
                        _ => unreachable!(),
                    }
                }
                if me.eq(&"Y") {
                    score_2 += match opp {
                        &"A" => 1,
                        &"B" => 2,
                        &"C" => 3,
                        _ => unreachable!(),
                    }
                }
                if me.eq(&"Z") {
                    score_2 += match opp {
                        &"A" => 2,
                        &"B" => 3,
                        &"C" => 1,
                        _ => unreachable!(),
                    }
                }
            }
        }
    }
    println!("Part 2: {}", score_2);
}

fn day_3() {
    println!("Day 3");
    let score_lookup = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";

    let mut score_1: usize = 0;
    let mut score_2: usize = 0;
    if let Ok(lines) = read_lines("./inputs/day_3.txt") {
        let mut group: Vec<String> = Vec::new();
        for line in lines {
            if let Ok(backpack) = line {
                group.push(backpack.clone());
                if group.len() == 3 {
                    let badge = common_char_two(&group[0], &group[1], &group[2]);
                    score_2 += score_lookup.find(badge).unwrap() + 1;
                    group = Vec::new();
                }
                let (a, b) = backpack.split_at(backpack.len() / 2);
                let common = common_char(a, b);
                score_1 += score_lookup.find(common).unwrap() + 1;
            }
        }
    }
    println!("Part 1: {}", score_1);
    println!("Part 2: {}", score_2);
}

fn common_char(a: &str, b: &str) -> char {
    let set_a: HashSet<char> = a.chars().collect();
    let set_b: HashSet<char> = b.chars().collect();
    *set_a.intersection(&set_b).next().unwrap()
}
fn common_char_two(a: &str, b: &str, c: &str) -> char {
    let set_a: HashSet<char> = a.chars().collect();
    let set_b: HashSet<char> = b.chars().collect();
    let set_c: HashSet<char> = c.chars().collect();
    *set_a.intersection(&set_b).cloned().collect::<HashSet<char>>().intersection(&set_c).next().unwrap()
}


fn day_4() {
    println!("Day 4");
}
fn day_5() {
    println!("Day 5");
}
fn day_6() {
    println!("Day 6");
}
// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
