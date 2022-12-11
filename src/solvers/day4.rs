use std::cmp::{max, min};
use std::error::Error;
use std::fs;
use std::ops::Range;

pub fn day4(any_overlap: bool) -> Result<(), Box<dyn Error>> {
    let mut contained_count = 0;
    for line in fs::read_to_string("inputs/day4.txt")?.lines() {
        let ranges: Vec<&str> = line.split(',').collect();
        if ranges.len() != 2 {
            return Err(Box::from(format!("expected 2 ranges, but got {}", ranges.len())));
        }
        let left_range = parse_range(ranges[0])?;
        let right_range = parse_range(ranges[1])?;

        let overlap = get_overlap(&left_range, &right_range);
        if overlap == None {
            continue;
        }
        if any_overlap
            || overlap == Some(left_range.len())
            || overlap == Some(right_range.len()) {
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

pub fn get_overlap(left: &Range<usize>, right: &Range<usize>) -> Option<usize> {
    let left = left.start..left.end + 1;
    let right = right.start..right.end + 1;
    let overlap_start = max(left.start, right.start);
    let overlap_end = min(left.end, right.end);
    if overlap_start < overlap_end {
        Some(overlap_end - overlap_start - 1)
    } else { None }
}
