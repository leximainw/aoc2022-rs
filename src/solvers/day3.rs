use std::error::Error;
use std::fs;

const PRIORITY_MAP: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";

pub fn day3() -> Result<(), Box<dyn Error>> {
    let mut priority_sum = 0;
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

        let mut last = None;
        while common.len() != 0 {
            let curr = common.pop();
            if curr == None {
                break;
            }
            if curr != last {
                last = curr;
                if let Some(index) = PRIORITY_MAP.find(curr.unwrap()) {
                    priority_sum += index + 1;
                } else {
                    return Err(Box::from(format!("found unexpected common item {}", curr.unwrap())));
                }
            }
        }
    }

    println!("{priority_sum}");
    Ok(())
}
