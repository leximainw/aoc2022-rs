mod solvers;

use solvers::{
    day1, day2, day3,
};

use std::error::Error;
use std::io::{
    BufRead,
    stdin,
    stdout,
    Write,
};

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
            "3a" => day3()?,
            "exit" => break,
            _ => println!("Unknown option {value}"),
        };
    }
    Ok(())
}
