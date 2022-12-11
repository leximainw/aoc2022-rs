use std::error::Error;
use std::fs;

const PRIORITY_MAP: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";

pub fn day3(num_groups: usize, per_line: bool) -> Result<(), Box<dyn Error>> {
    let mut priority_sum = 0;
    let mut groups: Vec<Vec<char>> = Vec::new();
    for line in fs::read_to_string("inputs/day3.txt")?.lines() {
        if per_line {
            let chars: Vec<char> = line.chars().collect();
            let group_size = chars.len() / num_groups;
            for group_id in 0..num_groups {
                let group_start = group_id * group_size;
                let mut group = chars[group_start..group_start + group_size]
                    .iter().map(|x| *x).collect::<Vec<char>>();
                if group.len() == 0 {
                    continue;
                }
                group.sort();
                groups.push(group);
            }
        } else {
            let mut group = line.chars().collect::<Vec<char>>();
            if group.len() == 0 {
                continue;
            }
            group.sort();
            groups.push(group);
            if groups.len() != num_groups {
                continue;
            }
        }
        let mut common = Vec::<char>::new();
        loop {
            let mut max_value = None;
            let mut max_index = None;
            for i in 0..groups.len() {
                let group = &groups[i];
                let next = group[group.len() - 1];
                if max_value == None || Some(next) > max_value {
                    max_value = Some(next);
                    max_index = Some(i);
                }
            }
            let max_group = groups.remove(max_index.unwrap());
            if max_group.len() == 0 {
                break;
            }
            let test_char = max_group[max_group.len() - 1];
            groups.push(max_group);
            let mut test_count = 0;
            for i in 0..groups.len() {
                let mut group = &mut groups[i];
                if group[group.len() - 1] == test_char {
                    test_count += 1;
                    group.pop();
                }
            }
            if test_count == num_groups {
                common.push(test_char);
            }
            if groups.iter().any(|x| x.len() == 0) {
                break;
            }
        }
        groups.clear();

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
