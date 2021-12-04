use std::fs::read_to_string;

fn main() {
    let input = read_to_string("./input.txt").expect("failed to open input file");
    let mut lines: Vec<_> = input.split("\n").filter(|s| s != &"").collect();

    let calls: Vec<i32> = lines
        .remove(0)
        .split(",")
        .map(|s| s.parse().unwrap())
        .collect();

    let mut cards: Vec<Card> = Vec::new();

    for group in 0..(lines.len() / 5) {
        cards.push(Card::new(&lines[group * 5..(group * 5) + 5], group));
    }

    for call in calls {
        for card in &mut cards {
            card.mark(call);
        }

        if cards.len() > 1 {
            cards.retain(|card| !card.bingo());
        } else {
            if cards[0].bingo() {
                println!("{}", cards[0].score() * call);
                return;
            }
        }
    }
}

#[derive(Debug)]
struct Card {
    pub index: usize,
    lines: Vec<Vec<i32>>,
}

impl Card {
    pub fn new(lines: &[&str], index: usize) -> Card {
        let mut stored_lines = Vec::new();
        for line in lines {
            let values: Vec<i32> = line
                .trim()
                .split(" ")
                .filter(|s| s != &"")
                .map(|s| s.parse().unwrap())
                .collect();
            stored_lines.push(values);
        }
        Card {
            index,
            lines: stored_lines,
        }
    }

    pub fn mark(&mut self, call: i32) {
        for i in 0..self.lines.len() {
            for j in 0..self.lines[i].len() {
                if self.lines[i][j] == call {
                    self.lines[i][j] = -1
                }
            }
        }
    }

    pub fn bingo(&self) -> bool {
        for line in &self.lines {
            if line.iter().sum::<i32>() == -5 {
                return true;
            }
        }

        for j in 0..self.lines[0].len() {
            let mut sum = 0;
            for i in 0..self.lines.len() {
                sum += self.lines[i][j];
            }
            if sum == -5 {
                return true;
            }
        }

        false
    }

    pub fn score(&self) -> i32 {
        let mut sum = 0;

        for line in &self.lines {
            for number in line {
                if number > &-1 {
                    sum += number;
                }
            }
        }

        sum
    }
}
