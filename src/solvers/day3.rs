use std::error::Error;
use std::fs;

pub fn day3() -> Result<(), Box<dyn Error>> {
    for line in fs::read_to_string("inputs/day3.txt")?.lines() {
        let chars: Vec<char> = line.chars().collect();
        let mut left = chars[0..chars.len() / 2].iter().collect::<Vec<&char>>();
        let mut right = chars[chars.len() / 2..chars.len()].iter().collect::<Vec<&char>>();
        let mut common = Vec::<char>::new();
        left.sort();
        right.sort();
        while left.len() != 0 && right.len() != 0 {
            let left_next = left[left.len() - 1];
            let right_next = right[right.len() - 1];
            if left_next < right_next {
                right.pop();
            } else if left_next > right_next {
                left.pop();
            } else {
                common.push(*left_next);
                left.pop();
                right.pop();
            }
        }
        println!("{common:?}")
    }
    Ok(())
}