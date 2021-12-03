use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

fn main() {
    let input = read_input("./input.txt");
    let row_count = input.len();
    let width = input[0].len();

    let mut gamma = String::new();
    let mut epsilon = String::new();

    for i in 0..width {
        let mut ones_count = 0;
        for line in &input {
            if line.chars().nth(i).unwrap() == '1' {
                ones_count += 1;
            }
        }
        if ones_count > row_count / 2 {
            gamma += "1";
            epsilon += "0";
        } else {
            gamma += "0";
            epsilon += "1";
        }
    }

    let gamm_dec = isize::from_str_radix(&gamma, 2).unwrap();
    let epsi_dec = isize::from_str_radix(&epsilon, 2).unwrap();

    println!("{}", gamm_dec);
    println!("{}", epsi_dec);
    println!("{}", gamm_dec * epsi_dec);
}

fn read_input(dir: &str) -> Vec<String> {
    let f = File::open(dir).unwrap();
    let r = BufReader::new(f);

    r.lines().map(|line| line.unwrap()).collect()
}
