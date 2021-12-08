use std::collections::HashMap;
use std::fs::read_to_string;

fn main() {
    let input = read_to_string("./input.txt").expect("failed to open input file");
    let input_strings = input.split('\n');

    let signal_patterns = input_strings
        .map(|s| {
            let parts: Vec<_> = s.split(" | ").collect();
            let left = parts[0].split(' ').collect();
            let right = parts[1].split(' ').collect();
            (left, right)
        })
        .collect();

    println!("{}", find_output(signal_patterns));
}

fn find_output(signal_patterns: Vec<(Vec<&str>, Vec<&str>)>) -> usize {
    let mut count = 0;
    for (pattern, output) in signal_patterns {
        let mut value_map = HashMap::<&str, _>::new();

        let one_pattern = pattern.iter().find(|x| x.len() == 2).unwrap();
        value_map.insert(one_pattern, "1");

        let seven_pattern = pattern.iter().find(|x| x.len() == 3).unwrap();
        value_map.insert(seven_pattern, "7");

        let eight_pattern = pattern.iter().find(|x| x.len() == 7).unwrap();
        value_map.insert(eight_pattern, "8");

        let four_pattern = pattern.iter().find(|x| x.len() == 4).unwrap();
        value_map.insert(four_pattern, "4");

        let nine_pattern = pattern
            .iter()
            .find(|x| (x.len() == 6) & (intersect(x, four_pattern) == 4))
            .unwrap();
        value_map.insert(nine_pattern, "9");

        let zero_pattern = pattern
            .iter()
            .find(|x| (x.len() == 6) & (intersect(x, seven_pattern) == 3) & (x != &nine_pattern))
            .unwrap();
        value_map.insert(zero_pattern, "0");

        let six_pattern = pattern
            .iter()
            .find(|x| (x.len() == 6) & (x != &zero_pattern) & (x != &nine_pattern))
            .unwrap();
        value_map.insert(six_pattern, "6");

        let five_pattern = pattern
            .iter()
            .find(|x| (x.len() == 5) & (intersect(x, six_pattern) == 5))
            .unwrap();
        value_map.insert(five_pattern, "5");

        let three_pattern = pattern
            .iter()
            .find(|x| (x.len() == 5) & (intersect(x, four_pattern) == 3) & (x != &five_pattern))
            .unwrap();
        value_map.insert(three_pattern, "3");

        let two_pattern = pattern
            .iter()
            .find(|x| (x.len() == 5) & (x != &three_pattern) & (x != &five_pattern))
            .unwrap();
        value_map.insert(two_pattern, "2");

        let mut str_val = String::new();

        for pattern in output {
            for (key, val) in &value_map {
                if (pattern.len() == intersect(key, pattern)) & (pattern.len() == key.len()) {
                    str_val += val;
                }
            }
        }

        let int_val: usize = str_val.parse().unwrap();

        count += int_val
    }

    count
}

use std::collections::HashSet;

fn intersect(a: &str, b: &str) -> usize {
    let (shorter, longer) = if a.len() > b.len() { (b, a) } else { (a, b) };
    let set: HashSet<char> = shorter.chars().collect();

    let mut count = 0;
    for c in longer.chars() {
        if set.contains(&c) {
            count += 1;
        }
    }
    count
}
