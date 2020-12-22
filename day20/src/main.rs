use std::io::{self, BufRead};
use std::collections::HashMap;

fn bools_to_fingerprint<I>(bools : I) -> usize
    where I : Iterator<Item = bool>
{
    let mut result = 0;
    for b in bools {
        result *= 2;
        if b == true {
            result += 1;
        }
    }
    return result;
}

fn inv10(mut n : usize) -> usize {
   let mut m : usize = 0;
   for _ in 0..10 {
     m = m << 1 | (n & 1);
     n = n >> 1;
   }
   return m;
}

fn rot1(prints : (usize, usize, usize, usize)) -> (usize, usize, usize, usize) {
    let (top, right, bottom, left) = prints;
    return (left, top, right, bottom);
}

fn rotn(prints : (usize, usize, usize, usize)) -> impl Iterator<Item = (usize, usize, usize, usize)>
{
    (0..4).scan(prints, |st, _| {
        let now = *st;
        *st = rot1(*st);
        return Some(now);
    })
}

fn tile_fingerprints(tile : &Vec<Vec<bool>>) -> Vec<(usize, usize, usize, usize)> {
    let top = bools_to_fingerprint(tile[0].iter().map(|b| *b));
    let right = bools_to_fingerprint(tile.iter().map(|l| l[l.len() - 1]));
    let bottom = bools_to_fingerprint(tile[tile.len() - 1].iter().rev().map(|b| *b));
    let left = bools_to_fingerprint(tile.iter().rev().map(|l| l[0]));

    vec!(
        (top,           right,        bottom,        left),
        (inv10(top),    inv10(left),  inv10(bottom), inv10(right)), // flip along horizontal axis
        (inv10(bottom), inv10(right), inv10(top),    inv10(left)),  // flip along vertical axis
        (top,           left,         bottom,        right)  // flip both
    )
}

fn explore(grid_size : usize, tiles_by_fingerprints : &HashMap<usize, Vec<(usize, (usize, usize, usize, usize))>>, arrangement : &mut Vec<(usize, (usize, usize, usize, usize))>) {
    //if arrangement.len() > 7 {
    //    println!("{} {}", arrangement.len(), arrangement.iter().map(|(t, _)| t.to_string()).collect::<Vec<String>>().join(" "));
    //}
    //if arrangement.len() == 8 {
    //    for (n, prints) in arrangement.iter() {
    //      println!("  {:?} {:?}", n, &prints);
    //    }
    //}
    if arrangement.len() == grid_size * grid_size {
        dbg!(grid_size);
        dbg!(arrangement[0].0, arrangement[grid_size - 1].0, arrangement[arrangement.len() - 1].0, arrangement[arrangement.len() - grid_size - 1].0);
    }
    if arrangement.len() == 0 {
        for (number, prints) in tiles_by_fingerprints.values().map(|v| v.iter()).flatten() {
            arrangement.push((*number, *prints));
            explore(grid_size, tiles_by_fingerprints, arrangement);
            arrangement.pop();
        }
    } else if arrangement.len() < grid_size {
        for (number, prints) in tiles_by_fingerprints.get(&inv10(arrangement[arrangement.len()-1].1.2)).unwrap_or(&vec!()) {
            if arrangement.iter().find(|(n, _)| n == number).is_none() {
                arrangement.push((*number, *prints));
                explore(grid_size, tiles_by_fingerprints, arrangement);
                arrangement.pop();
            }
        }
    } else if arrangement.len() % grid_size == 0 {
        for (number, prints) in tiles_by_fingerprints.get(&inv10(arrangement[arrangement.len() - grid_size].1.3)).unwrap_or(&vec!()) {
            if arrangement.iter().find(|(n, _)| n == number).is_none() {
                arrangement.push((*number, rot1(*prints)));
                explore(grid_size, tiles_by_fingerprints, arrangement);
                arrangement.pop();
            }
        }
    } else {
        for (number, prints) in tiles_by_fingerprints.get(&inv10(arrangement[arrangement.len()-1].1.2)).unwrap_or(&vec!()) {
            //if arrangement.len() == 8 {
            //    dbg!(number, prints, (inv10(prints.0), inv10(prints.1), inv10(prints.2), inv10(prints.3)), arrangement[arrangement.len() - grid_size].1.3, prints.1);
            //}
            if arrangement.iter().find(|(n, _)| n == number).is_none() && inv10(arrangement[arrangement.len() - grid_size].1.3) == prints.1 {
                arrangement.push((*number, *prints));
                explore(grid_size, tiles_by_fingerprints, arrangement);
                arrangement.pop();
            }
        }

    }
}

fn main() {
    let mut current_tile_id : usize = 0;
    let mut current_tile : Vec<Vec<bool>> = Vec::new();
    let mut tiles : Vec<(usize, Vec<Vec<bool>>)> = Vec::new();
    let mut tiles_by_fingerprints : HashMap<usize, Vec<(usize, (usize, usize, usize, usize))>> = HashMap::new();
    for wrapped_line in io::stdin().lock().lines() {
        let line = wrapped_line.unwrap();
        if line.len() == 0 {
            tiles.push((current_tile_id, current_tile));
            current_tile = Vec::new();
        } else if line.starts_with("Tile ") {
            current_tile_id = line[5..line.len()-1].parse::<usize>().unwrap();
        } else {
            current_tile.push(line.chars().map(|c| c == '#').collect());
        }
    }
    if current_tile.len() != 0 {
        tiles.push((current_tile_id, current_tile));
    }
    for (tile_number, tile) in &tiles {
        for print in tile_fingerprints(tile) {
         //   dbg!(tile_number, print);
            for rotation in rotn(print) {
                tiles_by_fingerprints.entry(rotation.0)
                    .or_insert(Vec::new())
                    .push((*tile_number, rotation));
            }
        }
    }

    let mut arrangement : Vec<(usize, (usize, usize, usize, usize))> = Vec::new();
    explore((tiles.len() as f32).sqrt() as usize, &tiles_by_fingerprints, &mut arrangement);
}
