use std::fs::read_to_string;

fn main() {
    let input = read_to_string("./input.txt").expect("failed to open input file");
    let input_strings = input.split('\n');

    let heights = input_strings
        .map(|s| s.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect();

    println!("{}", calc_risk_level(heights));
}

fn calc_risk_level(map: Vec<Vec<u32>>) -> u32 {
    let mut risk_level = 0;

    for (i, row) in map.iter().enumerate() {
        for (j, point) in row.iter().enumerate() {
            let i = i as i32;
            let j = j as i32;

            let width = row.len() as i32;
            let height = map.len() as i32;
            let neighbours = find_neighbours((i, j), width, height);

            let mut is_low = true;

            for p in neighbours {
                let (x, y) = p;
                let neighbour = map[x as usize][y as usize];

                if point >= &neighbour {
                    is_low = false;
                    break;
                }
            }

            if is_low {
                risk_level += point + 1
            }
        }
    }

    risk_level
}

fn find_neighbours(point: (i32, i32), width: i32, height: i32) -> Vec<(i32, i32)> {
    let (i, j) = point;
    let candidates: Vec<(i32, i32)> = vec![(i + 1, j), (i - 1, j), (i, j + 1), (i, j - 1)];

    candidates
        .into_iter()
        .filter(|c| {
            let (x, y) = c;
            (x >= &0) & (x < &height) & (y >= &0) & (y < &width)
        })
        .collect()
}