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
                    flash_stack.push((i as i32, j as i32))
                }
            }
        }

        while let Some(walk_point) = flash_stack.pop() {
            let i = walk_point.0 as usize;
            let j = walk_point.1 as usize;

            if has_flashed.contains(&(i, j)) {
                continue;
            }

            has_flashed.insert((i, j));

            let neighbours = find_neighbours(walk_point, width, height);

            for neighbour in neighbours {
                let n_i = neighbour.0 as usize;
                let n_j = neighbour.1 as usize;

                energy_levels[n_i][n_j] = energy_levels[n_i][n_j] + 1;

                if energy_levels[n_i][n_j] > 9 {
                    flash_stack.push(neighbour)
                }
            }
        }

        for entry in has_flashed {
            let n_i = entry.0 as usize;
            let n_j = entry.1 as usize;
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

fn find_neighbours(point: (i32, i32), width: i32, height: i32) -> Vec<(i32, i32)> {
    let (i, j) = point;
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
        .collect()
}
