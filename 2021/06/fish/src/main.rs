use std::fs::read_to_string;

fn main() {
    let input = read_to_string("./input.txt").expect("failed to open input file");
    let input_strings = input.split(",");

    let lines = input_strings.map(|s| s.parse().unwrap()).collect();

    // let lines = vec![3, 4, 3, 1, 2];
    println!("{}", count_fish(lines));
}

fn count_fish(start: Vec<usize>) -> u64 {
    let mut counts = [0u64; 9];

    for day in start {
        counts[day] += 1;
    }

    for _ in 1..=256 {
        let zero_count = counts[0];
        for num in 1..=8 {
            let today_count = counts[num];
            counts[num - 1] = today_count;
        }
        counts[8] = zero_count;
        counts[6] = counts[6] + zero_count;
    }

    counts.iter().sum()
}
