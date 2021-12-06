use std::fs::read_to_string;

fn main() {
    let input = read_to_string("./input.txt").expect("failed to open input file");
    let input_strings = input.split(",");

    let lines = input_strings.map(|s| s.parse().unwrap()).collect();

    println!("{}", count_fish(lines));
}

fn count_fish(start: Vec<usize>) -> u64 {
    let mut counts = [0u64; 9];

    for day in start {
        counts[day] += 1;
    }

    for _ in 0..256 {
        counts.rotate_left(1);
        counts[6] += counts[8];
    }

    counts.iter().sum()
}
