use std::error::Error;
use std::fs;
use std::str::Lines;

pub fn day1(best_n: usize) -> Result<(), Box<dyn Error>> {
    let elves = read_elves(fs::read_to_string("inputs/day1.txt")?.lines())?;
    let mut sum = 0;
    for i in 0..best_n {
        sum += elves[i].1
    }
    println!(
        "The top {} carrying {} calories.",
        if best_n == 1 { format!("elf is") }
        else { format!("{} elves are", best_n) },
        sum,
    );
    Ok(())
}

fn read_elves(lines: Lines) -> Result<Vec<(i32, i32)>, Box<dyn Error>> {
    let mut elves = Vec::new();
    let mut sum = 0;
    let mut idx = 0;
    for line in lines {
        if line.len() == 0 {
            elves.push((idx, sum));
            sum = 0;
            idx += 1;
        } else {
            sum += line.parse::<i32>()?;
        }
    }
    elves.sort_by(|l, r| r.1.partial_cmp(&l.1).unwrap());
    Ok(elves)
}
