use std::error::Error;
use std::fs;

pub fn day2(know_meaning: bool) -> Result<(), Box<dyn Error>> {
    let mut total_score = 0;
    for line in fs::read_to_string("inputs/day2.txt")?.lines() {
        if line == "" {
            continue;
        }
        let words: Vec<&str> = line.split_whitespace().collect();
        if words.len() != 2 {
            return Err(Box::from("expect two words per line"));
        }
        let against = match words[0] {
            "A" => 0,
            "B" => 2,
            "C" => 1,
            _ => return Err(Box::from("expect A, B, or C for opponent's choice")),
        };
        let choice = if know_meaning {
            (match words[1] {
                "X" => 2 - against,
                "Y" => (3 - against) % 3,
                "Z" => (4 - against) % 3,
                _ => return Err(Box::from("expect X, Y, or Z for player's choice")),
            } + 1)
        } else {
            match words[1] {
                "X" => 1,
                "Y" => 2,
                "Z" => 3,
                _ => return Err(Box::from("expect X, Y, or Z for player's choice")),
            }
        };
        total_score += choice + (choice + against) % 3 * 3;
    }
    println!("Your final score will be {total_score}.");
    Ok(())
}
