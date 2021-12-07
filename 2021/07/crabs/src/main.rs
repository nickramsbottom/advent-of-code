use std::fs::read_to_string;

fn main() {
    let input = read_to_string("./input.txt").expect("failed to open input file");
    let input_strings = input.split(",");

    let positions = input_strings.map(|s| s.parse().unwrap()).collect();

    println!("{}", find_fuel(positions));
}

fn find_fuel(mut positions: Vec<i32>) -> i32 {
    positions.sort();
    let length = positions.len();
    let median = positions[length / 2];

    let mut fuel: i32 = 0;

    for pos in positions {
        let diff = (median - pos).abs();
        fuel += diff;
    }

    fuel
}
