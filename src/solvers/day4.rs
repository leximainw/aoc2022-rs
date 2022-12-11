use std::error::Error;
use std::fs;
use std::ops::Range;

pub fn day4() -> Result<(), Box<dyn Error>> {
    let mut contained_count = 0;
    for line in fs::read_to_string("inputs/day4.txt")?.lines() {
        let ranges: Vec<&str> = line.split(',').collect();
        if ranges.len() != 2 {
            return Err(Box::from(format!("expected 2 ranges, but got {}", ranges.len())));
        }
        let left_range = parse_range(ranges[0])?;
        let right_range = parse_range(ranges[1])?;
        if left_range.start >= right_range.start && left_range.end <= right_range.end
            || left_range.start <= right_range.start && left_range.end >= right_range.end {
            contained_count += 1;
        }
    }
    println!("{contained_count}");
    Ok(())
}

pub fn parse_range(range: &str) -> Result<Range<usize>, Box<dyn Error>> {
    let endpoints: Vec<&str> = range.split('-').collect();
    if endpoints.len() != 2 {
        return Err(Box::from(format!("expected 2 endpoints, but got {}", endpoints.len())));
    }
    Ok(str::parse(endpoints[0])?..str::parse(endpoints[1])?)
}
