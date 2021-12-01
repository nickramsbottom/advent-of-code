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

    let mut sum = ns[0] + ns[1] + ns[2];
    let mut c = 0;
    for w in ns.windows(3).skip(1) {
        let new_sum = w.iter().sum();

        if new_sum > sum {
            c += 1;
        }

        sum = new_sum;
    }

    println!("{}", c)
}
