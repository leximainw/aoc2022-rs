mod day1;
mod day2;

use std::error::Error;
use std::io::{
    BufRead,
    stdin,
    stdout,
    Write,
};

use day1::day1;
use day2::day2;

fn main() -> Result<(), Box<dyn Error>> {
    loop {
        print!("Select day: ");
        stdout().flush()?;
        let value = stdin().lock().lines().next().ok_or("unable to read from stdin")??;
        match value.as_str() {
            "1a" => day1(1)?,
            "1b" => day1(3)?,
            "2a" => day2(false)?,
            "2b" => day2(true)?,
            "exit" => break,
            _ => println!("Unknown option {value}"),
        };
    }
    Ok(())
}
