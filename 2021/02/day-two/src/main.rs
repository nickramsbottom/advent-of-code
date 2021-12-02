use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

enum Dir {
    Forward,
    Down,
    Up,
}

fn main() {
    let f = File::open("./input.txt").unwrap();
    let r = BufReader::new(f);

    let ns: Vec<(Dir, i32)> = r
        .lines()
        .map(|line| {
            let line = line.unwrap();
            let parts = line.split(' ').collect::<Vec<&str>>();
            let amt = parts[1].parse::<i32>().unwrap();

            let dir = match parts[0] {
                "forward" => Dir::Forward,
                "down" => Dir::Down,
                "up" => Dir::Up,
                _ => panic!("Invalid direction"),
            };

            (dir, amt)
        })
        .collect();

    let mut depth = 0;
    let mut hor = 0;
    let mut aim = 0;

    for n in ns {
        let (dir, amt) = n;

        match dir {
            Dir::Forward => {
                hor += amt;
                depth += aim * amt
            }
            Dir::Down => aim += amt,
            Dir::Up => aim -= amt,
        }
    }

    println!("{:?}", depth * hor)
}
