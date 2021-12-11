use std::collections::HashSet;
use std::fs::read_to_string;

fn main() {
    let input = read_to_string("./input.txt").expect("failed to open input file");
    let input_strings = input.split('\n');

    let mut energy_levels: Vec<Vec<i32>> = input_strings
        .map(|s| s.chars().map(|c| c.to_digit(10).unwrap() as i32).collect())
        .collect();

    let flashes = flash(&mut energy_levels);

    println!("{:?}", flashes)
}

fn flash(energy_levels: &mut Vec<Vec<i32>>) -> i32 {
    let width = energy_levels[0].len() as i32;
    let height = energy_levels.len() as i32;

    let mut step = 0;

    loop {
        step += 1;

        let mut flash_stack = Vec::new();
        let mut has_flashed = HashSet::new();
        for (i, row) in &mut energy_levels.into_iter().enumerate() {
            for (j, level) in row.into_iter().enumerate() {
                *level = *level + 1;

                if *level > 9 {
                    flash_stack.push((i, j))
                }
            }
        }

        while let Some(walk_point) = flash_stack.pop() {
            if has_flashed.contains(&walk_point) {
                continue;
            }

            has_flashed.insert(walk_point);

            let neighbours = find_neighbours(walk_point, width, height);

            for (n_i, n_j) in neighbours {
                energy_levels[n_i][n_j] = energy_levels[n_i][n_j] + 1;

                if energy_levels[n_i][n_j] > 9 {
                    flash_stack.push((n_i, n_j))
                }
            }
        }

        for (n_i, n_j) in has_flashed {
            energy_levels[n_i][n_j] = 0;
        }

        if all_zeros(energy_levels) {
            return step;
        }
    }
}

fn all_zeros(energy_levels: &mut Vec<Vec<i32>>) -> bool {
    for row in energy_levels {
        for num in row {
            if *num != 0 {
                return false;
            }
        }
    }

    return true;
}

fn find_neighbours(point: (usize, usize), width: i32, height: i32) -> Vec<(usize, usize)> {
    let i = point.0 as i32;
    let j = point.1 as i32;
    let candidates: Vec<(i32, i32)> = vec![
        (i + 1, j),
        (i - 1, j),
        (i, j + 1),
        (i, j - 1),
        (i + 1, j + 1),
        (i + 1, j - 1),
        (i - 1, j + 1),
        (i - 1, j - 1),
    ];

    candidates
        .into_iter()
        .filter(|c| {
            let (x, y) = c;
            (x >= &0) & (x < &height) & (y >= &0) & (y < &width)
        })
        .map(|(x, y)| (x as usize, y as usize))
        .collect()
}
