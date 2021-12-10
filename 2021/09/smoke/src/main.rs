use std::fs::read_to_string;
use std::collections::HashSet;

fn main() {
    let input = read_to_string("./input.txt").expect("failed to open input file");
    let input_strings = input.split('\n');

    let heights = input_strings
        .map(|s| s.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect();
    
    let low_points = calc_low_points(&heights);

    let mut seen = HashSet::new();

    let mut sizes: Vec<u32> = low_points.into_iter().map(|low_point| walk_valley(low_point, &heights, &mut seen)).collect();
    sizes.sort();
    let result: u32 = sizes.into_iter().rev().take(3).product();
    println!("{}", result);
    println!("{}", calc_risk_level(heights));
}

fn walk_valley(point: (i32, i32), map: &Vec<Vec<u32>>, seen: &mut HashSet<(i32, i32)>) -> u32 {
    let x = point.0 as usize;
    let y = point.1 as usize;
    let mut count = 1;
    let value = map[x][y];

    if value == 9 {
        return 0;
    }

    seen.insert(point);
    let width = map[0].len() as i32;
    let height = map.len() as i32;
    find_neighbours(point, width, height)
        .iter()
        .for_each(|neigh| {
            if !seen.contains(&neigh) {
                count += walk_valley(*neigh, map, seen)
            }
        });

    count
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