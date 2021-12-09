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
    let low_points = calc_low_points(&map);

    let mut risk_level = 0;

    for p in low_points {
        let x = p.0 as usize;
        let y = p.1 as usize;

        risk_level += map[x][y] + 1
    }

    risk_level
}

fn calc_low_points(map: &Vec<Vec<u32>>) -> Vec<(i32, i32)> {
    let mut low_points = Vec::new();

    for (i, row) in map.iter().enumerate() {
        for (j, point) in row.iter().enumerate() {
            let i = i as i32;
            let j = j as i32;

            let width = row.len() as i32;
            let height = map.len() as i32;
            let neighbours = find_neighbours((i, j), width, height);
            let is_low = calc_low(point, neighbours, &map);

            if is_low {
                low_points.push((i, j));
            }
        }
    }

    low_points
}

fn calc_low (point: &u32, neighbours: Vec<(i32, i32)>, map:&Vec<Vec<u32>>) -> bool {
    for p in neighbours {
        let (x, y) = p;
        let neighbour = map[x as usize][y as usize];

        if point >= &neighbour {
            return false
        }
    }
    return true
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