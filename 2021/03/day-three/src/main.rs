use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

fn main() {
    let input = read_input("./input.txt");
    // println!("{}", part_1(input));
    let one = isize::from_str_radix(&part_2(input.clone(), false), 2).unwrap();
    let two = isize::from_str_radix(&part_2(input.clone(), true), 2).unwrap();
    println!("part 2: {} {} = {}", one, two, one * two);
}

fn part_2(mut input: Vec<String>, invert: bool) -> String {
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

        let bit = if invert ^ (zeroes <= ones) { '1' } else { '0' };
        input = input
            .iter()
            .filter(|line| line.chars().nth(pos).unwrap() == bit)
            .cloned()
            .collect();
        pos += 1;
    }

    out
}

fn read_input(dir: &str) -> Vec<String> {
    let f = File::open(dir).unwrap();
    let r = BufReader::new(f);

    r.lines().map(|line| line.unwrap()).collect()
}
