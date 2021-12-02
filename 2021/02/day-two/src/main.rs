use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

fn main() {
    let f = File::open("./input.txt").unwrap();
    let r = BufReader::new(f);

    let ns: Vec<String> = r.lines().map(|line| line.unwrap()).collect();

    let mut depth = 0;
    let mut hor = 0;

    for n in ns {
        let split = n.split(' ').collect::<Vec<&str>>();
        let dir = split[0];
        let amt = split[1].parse::<i32>().unwrap();

        match dir {
            "forward" => hor += amt,
            "down" => depth += amt,
            "up" => depth -= amt,
            _ => println!("Invalid input"),
        }
    }

    println!("{:?}", depth * hor)
}
