mod day1;

use std::io::{
    BufRead,
    stdin,
    stdout,
    Write,
};

use day1::day1;

fn main() {
    loop {
        print!("Select day: ");
        stdout().flush().unwrap();
        let value = stdin().lock().lines().next().unwrap().unwrap();
        match value.as_str() {
            "1a" => day1(1),
            "1b" => day1(3),
            "exit" => break,
            _ => Ok(println!("Unknown option {value}")),
        };
    }
}
