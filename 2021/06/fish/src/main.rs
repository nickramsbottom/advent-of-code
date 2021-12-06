use std::collections::HashMap;
use std::fs::read_to_string;

fn main() {
    let input = read_to_string("./input.txt").expect("failed to open input file");
    let input_strings = input.split(",");

    let lines = input_strings.map(|s| s.parse().unwrap()).collect();

    // let lines = vec![3, 4, 3, 1, 2];
    println!("{}", count_fish(lines));
}

fn count_fish(start: Vec<i64>) -> i64 {
    let mut counts = HashMap::new();

    for x in 0..=8 {
        counts.insert(x, 0);
    }

    for day in start {
        let count = counts.entry(day).or_insert(0);
        *count += 1;
    }

    for _ in 1..=80 {
        let zero_count = counts.get(&0).unwrap().clone();
        for num in 1..=8 {
            let today_count = counts.get(&num).unwrap().clone();
            counts.insert(num - 1, today_count);
        }
        counts.insert(8, zero_count);

        let six_count = counts.entry(6).or_insert(0);
        *six_count = *six_count + zero_count;
    }

    let mut count = 0;

    for x in 0..=8 {
        count += counts.get(&x).unwrap();
    }

    count
}
