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
    let mut numbers = HashMap::<&str, u8>::new();

    for (pattern, _output) in signal_patterns {
        let mut first_three_mapping = HashMap::<&str, &str>::new();

        for input in pattern.iter() {
            match input.len() {
                2 => numbers.insert(input, 1),
                3 => numbers.insert(input, 7),
                4 => numbers.insert(input, 4),
                7 => numbers.insert(input, 8),
                _ => None,
            };
        }

        for input in pattern.iter() {
            let len = input.len();

            if len < 3 {
                continue;
            }

            let first_three = &input[0..3];
            let complement = first_three_mapping.get(first_three);

            match complement {
                Some(complement) => {
                    let len = input.len();
                    if len == 5 {
                        numbers.insert(input, 5);
                        numbers.insert(complement, 6);
                    } else {
                        numbers.insert(input, 6);
                        numbers.insert(complement, 5);
                    }
                }
                None => {
                    first_three_mapping.insert(first_three, input);
                }
            }
        }
    }

    println!("{:?}", numbers);

    10
}
