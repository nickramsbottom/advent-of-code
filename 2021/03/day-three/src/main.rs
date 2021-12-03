use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

fn main() {
    let input = read_input("./input.txt");
    // println!("{}", part_1(input));
    println!(
        "part 2: {}",
        isize::from_str_radix(&part_2(input.clone()), 2).unwrap()
    );
    // println!("{}", part_2(input.clone()));
}

fn part_2(mut input: Vec<String>) -> String {
    let mut pos = 0;
    let mut out = String::new();

    while input.len() > 0 {
        if input.len() == 1 {
            out = input[0].clone();
            break;
        }

        let ones = input
            .iter()
            .filter(|line| line.chars().nth(pos).unwrap() == '1')
            .count();
        let zeroes = input.len() - ones;

        let bit = if ones >= zeroes { '0' } else { '1' };
        input = input
            .iter()
            .filter(|line| line.chars().nth(pos).unwrap() == bit)
            .cloned()
            .collect();
        pos += 1;
    }

    out
}

// 1719
// 2400

fn read_input(dir: &str) -> Vec<String> {
    let f = File::open(dir).unwrap();
    let r = BufReader::new(f);

    r.lines().map(|line| line.unwrap()).collect()
}
