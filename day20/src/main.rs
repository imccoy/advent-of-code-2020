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
    )
}

fn find_lines_from(line_length : usize, tiles_by_fingerprints : &HashMap<usize, Vec<(usize, (usize, usize, usize, usize))>>, line : &mut Vec<(usize, (usize, usize, usize, usize))>) -> Vec<Vec<(usize, (usize, usize, usize, usize))>> {
    if line.len() == line_length {
        return vec!(line.clone());
    }
    let end_of_line = line[line.len() - 1];
    let mut result : Vec<Vec<(usize, (usize, usize, usize, usize))>> = Vec::new();
    for (next_id, next_prints) in tiles_by_fingerprints.get(&inv10(end_of_line.1.2)).unwrap_or(&vec!()) {
        if line.iter().find(|(line_id, _)| line_id == next_id).is_none() {
            line.push((*next_id, *next_prints));
            result.extend(find_lines_from(line_length, tiles_by_fingerprints, line));
            line.pop();
        }
    }
    return result;
}

fn main() {
    let mut current_tile_id : usize = 0;
    let mut current_tile : Vec<Vec<bool>> = Vec::new();
    let mut tiles : Vec<(usize, Vec<Vec<bool>>)> = Vec::new();
    let mut tiles_by_fingerprints : HashMap<usize, Vec<(usize, (usize, usize, usize, usize))>> = HashMap::new();
    let mut fingerprints_by_tile : HashMap<usize, Vec<(usize, usize, usize, usize)>> = HashMap::new();
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
                fingerprints_by_tile.entry(*tile_number)
                    .or_insert(Vec::new())
                    .push(rotation);
            }
        }
    }

    let grid_size = (tiles.len() as f32).sqrt() as usize;
    let mut lines : Vec<Vec<(usize, (usize, usize, usize, usize))>> = Vec::new();
    
    for (tile_number, prints) in fingerprints_by_tile {
        for print in prints {
            lines.extend(find_lines_from(grid_size, &tiles_by_fingerprints, &mut vec!((tile_number, print))));
        }
    }
    dbg!(lines.iter().map(|line| line.iter().map(|t| t.0.to_string()).collect::<Vec<String>>().join(" ")).collect::<Vec<String>>());

    let mut lines_by_starting_cell : HashMap<usize, Vec<&Vec<(usize, (usize, usize, usize, usize))>>> = HashMap::new();
    for line in &lines {
        println!("{}", line[0].0);
        lines_by_starting_cell.entry(line[0].0)
            .or_insert(Vec::new())
            .push(line);
    }

    for (starting_cell, lines) in &lines_by_starting_cell {
        for (line_number, line) in lines.iter().enumerate() {
            for other_line in lines.iter() {
                if line.get(1..).unwrap().iter().find(|cell| other_line.iter().find(|other_cell| other_cell.0 == cell.0).is_some()).is_some() {
                    continue;
                }
                let mut cells : Vec<usize> = Vec::new();
                cells.extend(other_line.iter().map(|cell| cell.0));
                match fill_out_grid(&lines_by_starting_cell, line, other_line, &cells, 1) {
                    Some(last_row) => {
                        dbg!(other_line[0], other_line[other_line.len() - 1], last_row[0], last_row[last_row.len() - 1]);
                        dbg!(other_line[0].0 * other_line[other_line.len() - 1].0 * last_row[0].0 * last_row[last_row.len() - 1].0);
                    },
                    None => { /* do nothing */ }
                }
            }
        }
    }
}

fn fill_out_grid(lines_by_starting_cell : &HashMap<usize, Vec<&Vec<(usize, (usize, usize, usize, usize))>>>, column : &Vec<(usize, (usize, usize, usize, usize))>, row : &Vec<(usize, (usize, usize, usize, usize))>, cells : &Vec<usize>, row_number : usize) -> Option<Vec<(usize, (usize, usize, usize, usize))>> {
    if row_number == 12 {
        return Some(row.clone());
    }
    'candidates: for candidate in lines_by_starting_cell.get(&column[row_number].0).unwrap() {
        if candidate.iter().find(|line_elem| cells.contains(&line_elem.0)).is_some() {
            continue;
        }
        for ((_, (_, _, _, prev_bottom)), (_, (_, top, _, _))) in row.iter().zip(candidate.iter()) {
            if inv10(*prev_bottom) != *top {
                continue 'candidates;
            }
        }
        let mut new_cells = cells.clone();
        new_cells.extend(candidate.iter().map(|cell| cell.0));
        match fill_out_grid(lines_by_starting_cell, column, &candidate, &new_cells, row_number + 1) {
            Some(result) => return Some(result),
            _ => { /* do nothing */ }
        }
    }
    return None;
}
