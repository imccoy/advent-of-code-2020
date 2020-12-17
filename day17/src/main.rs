use std::collections::HashMap;
use std::collections::HashSet;
use std::io::{self, BufRead};

fn count_neighbours_with_offsets(coords : &Vec<i32>, neighbour_counts: &mut HashMap<Vec<i32>, usize>, offsets: Vec<i32>) {
    if coords.len() == offsets.len() { 
        if !offsets.iter().all(|o| *o == 0) {
            *(neighbour_counts.entry(offsets.iter().zip(coords).map(|(a,o)| a + o).collect()).or_insert(0)) += 1;
        }
    } else {
        for d in &[-1, 0, 1] {
            let mut offset_here = offsets.clone();
            offset_here.push(*d);
            count_neighbours_with_offsets(coords, neighbour_counts, offset_here);
        }
    }
}

fn count_neighbours(cube : &HashSet<Vec<i32>>) -> HashMap<Vec<i32>, usize> {
    let mut neighbour_counts : HashMap<Vec<i32>, usize> = HashMap::new();
    for coords in cube {
        count_neighbours_with_offsets(coords, &mut neighbour_counts, Vec::new());
    }
    neighbour_counts
}

fn run(dimensions: usize, initial_cube : HashSet<Vec<i32>>) {
    let mut cube : HashSet<Vec<i32>> = HashSet::new();
    for cell in initial_cube {
        let mut p = cell.clone();
        for _dim in 2..dimensions {
            p.push(0);
        }
        cube.insert(p);
    }
    for _generation in 0..6 {
        let neighbours = count_neighbours(&cube);
        let mut next_cube : HashSet<Vec<i32>> = HashSet::new();
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

fn main() {
    let mut cube : HashSet<Vec<i32>> = HashSet::new();
    for (y, wrapped_line) in io::stdin().lock().lines().enumerate() {
        let line = wrapped_line.unwrap();
        for (x, c) in line.chars().enumerate() {
            if c == '#' {
                let mut p = Vec::new();
                p.push(x as i32);
                p.push(y as i32);
                cube.insert(p);
            }
        }
    }

    run(3, cube.clone());
    run(4, cube);
}
