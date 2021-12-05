use std::collections::HashMap;
use std::fs::read_to_string;

fn main() {
    let input = read_to_string("./input.txt").expect("failed to open input file");
    let input_lines: Vec<_> = input.split("\n").collect();

    let lines: Vec<((i32, i32), (i32, i32))> = input_lines
        .iter()
        .map(|s| {
            let parts: Vec<_> = s.split(" -> ").collect();
            let start: Vec<i32> = parts[0].split(",").map(|s| s.parse().unwrap()).collect();
            let end: Vec<i32> = parts[1].split(",").map(|s| s.parse().unwrap()).collect();

            return ((start[0], start[1]), (end[0], end[1]));
        })
        .collect();
    println!("{}", occupied_points(lines));
}

fn occupied_points(lines: Vec<((i32, i32), (i32, i32))>) -> i32 {
    let mut points = HashMap::new();

    for line in lines {
        let ((x1, y1), (x2, y2)) = line;

        if x1 == x2 {
            let range = if y1 > y2 { y2..=y1 } else { y1..=y2 };
            for y in range {
                let count = points.entry((x1, y)).or_insert(0);
                *count += 1;
            }
            continue;
        }

        if y1 == y2 {
            let range = if x1 > x2 { x2..=x1 } else { x1..=x2 };
            for x in range {
                let count = points.entry((x, y1)).or_insert(0);
                *count += 1;
            }
            continue;
        }

        if (x1 != x2) & (y1 != y2) {
            let dx = if x1 < x2 { 1 } else { -1 };
            let dy = if y1 < y2 { 1 } else { -1 };

            let mut x = x1;
            let mut y = y1;

            while (x, y) != (x2 + dx, y2 + dy) {
                let count = points.entry((x, y)).or_insert(0);
                *count += 1;

                y += dy;
                x += dx;
            }
        }
    }

    let mut total_points = 0;

    for (_, count) in points {
        if count > 1 {
            total_points += 1;
        }
    }

    total_points
}
