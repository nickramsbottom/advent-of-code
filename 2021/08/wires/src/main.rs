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

    for (pattern, _output) in signal_patterns {
        for input in pattern {
            match input.len() {
                2 | 3 | 4 | 7 => count += 1,
                _ => (),
            }
        }
    }

    count
}
