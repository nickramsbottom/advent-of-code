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

    let mut c = 0;
    for i in 1..ns.len() {
        if ns[i] > ns[i - 1] {
            c += 1;
        }
    }

    println!("{}", c)
}
