use std::collections::HashMap;
use std::collections::HashSet;
use std::io::{self, BufRead};

fn count_neighbours(cube : &HashSet<(i32, i32, i32)>) -> HashMap<(i32, i32, i32), usize> {
    let mut neighbour_counts : HashMap<(i32, i32, i32), usize> = HashMap::new();
    for (x, y, z) in cube {
        for xd in &[-1, 0, 1] {
            for yd in &[-1, 0, 1] {
                for zd in &[-1, 0, 1] {
                    if *xd == 0 && *yd == 0 && *zd == 0 {
                        continue;
                    }
                    *(neighbour_counts.entry((x + xd, y + yd, z + zd)).or_insert(0)) += 1;
                }
            }
        }
    }
    neighbour_counts
}

fn main() {
    let mut cube : HashSet<(i32, i32, i32)> = HashSet::new();
    for (y, wrapped_line) in io::stdin().lock().lines().enumerate() {
        let line = wrapped_line.unwrap();
        for (x, c) in line.chars().enumerate() {
            if c == '#' {
                cube.insert((x as i32, y as i32, 0));
            }
        }
    }
    for generation in 0..6 {
        let neighbours = count_neighbours(&cube);
        let mut next_cube : HashSet<(i32, i32, i32)> = HashSet::new();
        for (coords, count) in neighbours {
            if cube.contains(&coords) {
                if count == 2 || count == 3 {
                    next_cube.insert(coords);
                }
            } else {
                if count == 3 {
                    next_cube.insert(coords);
                }
            }
        }
        cube = next_cube;
    }
    println!("{}", cube.len());
}
