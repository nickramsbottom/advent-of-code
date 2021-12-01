use itertools::Itertools;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

fn main() {
    let f = File::open("./input.txt").unwrap();
    let r = BufReader::new(f);

    let ns: Vec<usize> = r
        .lines()
        .map(|line| line.unwrap().parse::<usize>().unwrap())
        .collect();

    let n = ns
        .iter()
        .tuple_windows()
        .filter(|w| {
            let (a, _, _, d) = w;
            d > a
        })
        .count();

    println!("{}", n)
}
