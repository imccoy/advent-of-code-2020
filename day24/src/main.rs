use std::collections::HashMap;
use std::collections::HashSet;
use std::io::{self, BufRead};
use std::hash::Hash;

fn main() {
    let mut positions : HashMap<(i32, i32), usize> = HashMap::new();
    for wrapped_line in io::stdin().lock().lines() {
        let line = wrapped_line.unwrap();
        let mut position_x : i32 = 0;
        let mut position_y : i32 = 0;
        let mut cs = line.chars();
        while let Some(c) = cs.next() {
            if c == 'e' {
                position_x += 2;
            } else if c == 'w' {
                position_x -= 2;
            } else if c == 'n' {
                position_y -= 1;
                let c2 = cs.next().unwrap();
                if c2 == 'e' {
                    position_x += 1;
                } else if c2 == 'w' {
                    position_x -= 1;
                } else {
                    panic!("no what");
                }
            } else if c == 's' {
                position_y += 1;
                let c2 = cs.next().unwrap();
                if c2 == 'e' {
                    position_x += 1;
                } else if c2 == 'w' {
                    position_x -= 1;
                } else {
                    panic!("huh");
                }
            } else {
                panic!("um no");
            }
        }
        positions.entry((position_x, position_y))
            .and_modify(|n| *n += 1)
            .or_insert(1);
    }

    let mut currently_black : HashSet<(i32, i32)> = HashSet::new();
    currently_black.extend(positions.iter().filter_map(|(position, count)| if count % 2 == 0 { None } else { Some(position) }));
    println!("{}", currently_black.len());
    for day in 0..100 {
        let mut neighbour_counts : HashMap<(i32, i32), usize> = HashMap::new();
        for (x, y) in &currently_black {
            increment(&mut neighbour_counts, (x - 2, *y));
            increment(&mut neighbour_counts, (x + 2, *y));
            increment(&mut neighbour_counts, (x - 1, y + 1));
            increment(&mut neighbour_counts, (x - 1, y - 1));
            increment(&mut neighbour_counts, (x + 1, y + 1));
            increment(&mut neighbour_counts, (x + 1, y - 1));
        }
        let mut next : HashSet<(i32, i32)> = HashSet::new();
        for (position, neighbours) in neighbour_counts {
            if currently_black.contains(&position) {
                if neighbours == 1 || neighbours == 2 {
                    next.insert(position);
                }
            } else {
                if neighbours == 2 {
                    next.insert(position);
                }
            }
        }
        println!("{} {}", day + 1, next.len());
        currently_black = next;
    }
}

fn increment<K: Eq + Hash>(map : &mut HashMap<K, usize>, k : K) {
    map.entry(k)
        .and_modify(|n| *n += 1)
        .or_insert(1);
}
