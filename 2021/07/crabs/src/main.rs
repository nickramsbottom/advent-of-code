use std::fs::read_to_string;

fn main() {
    let input = read_to_string("./input.txt").expect("failed to open input file");
    let input_strings = input.split(",");

    let positions = input_strings.map(|s| s.parse().unwrap()).collect();

    println!("{}", find_fuel(positions));
}

fn find_fuel(positions: Vec<isize>) -> isize {
    let length = positions.len();
    let sum: isize = positions.iter().sum();
    let mean = (sum as f32 / length as f32).floor();
    let mut fuel = 0;

    for pos in positions {
        let diff = mean as isize - pos;
        let diff_pos = diff.abs();
        fuel += cost(diff_pos);
    }

    fuel
}

fn cost(dist: isize) -> isize {
    (dist * (dist + 1)) / 2
}
