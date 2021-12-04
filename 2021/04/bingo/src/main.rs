use std::fs::read_to_string;

fn main() {
    let input = read_to_string("./input.txt").expect("failed to open input file");
    let mut lines: Vec<_> = input.split("\n").filter(|s| s != &"").collect();

    let calls: Vec<u32> = lines
        .remove(0)
        .split(",")
        .map(|s| s.parse().unwrap())
        .collect();

    let mut cards: Vec<Card> = Vec::new();

    for group in 0..(lines.len() / 5) {
        cards.push(Card::new(&lines[group..group + 5]));
    }

    println!("{:?}", cards);
}

#[derive(Debug)]
struct Card {
    pub lines: Vec<Vec<u32>>,
}

impl Card {
    pub fn new(lines: &[&str]) -> Card {
        let mut stored_lines = Vec::new();
        for line in lines {
            let values: Vec<u32> = line
                .trim()
                .split(" ")
                .filter(|s| s != &"")
                .map(|s| s.parse().unwrap())
                .collect();
            stored_lines.push(values);
        }
        Card {
            lines: stored_lines,
        }
    }
}
