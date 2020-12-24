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

fn rotn(tile : Tile) -> impl Iterator<Item = Tile>
{
    (0..4).scan(tile.prints, move |st, n| {
        let mut tile = tile.clone();
        tile.prints = *st;
        tile.rotation = n;
        *st = rot1(*st);
        return Some(tile);
    })
}

#[derive(Clone, Debug)]
struct Tile {
    id : usize,
    prints : (usize, usize, usize, usize),
    rotation : usize,
    flipped : bool
}

fn tile_fingerprints(tile : &Vec<Vec<bool>>) -> (usize, usize, usize, usize) {
    let top = bools_to_fingerprint(tile[0].iter().map(|b| *b));
    let right = bools_to_fingerprint(tile.iter().map(|l| l[l.len() - 1]));
    let bottom = bools_to_fingerprint(tile[tile.len() - 1].iter().rev().map(|b| *b));
    let left = bools_to_fingerprint(tile.iter().rev().map(|l| l[0]));

    (left, top, right, bottom)
}

fn flip_prints(prints : (usize, usize, usize, usize)) -> (usize, usize, usize, usize) {
    let left = prints.0;
    let top = prints.1;
    let right = prints.2;
    let bottom = prints.3;

    (/*left*/   inv10(top),
     /*top*/    inv10(left),
     /*right*/  inv10(bottom),
     /*bottom*/ inv10(right)
    )
}

fn find_lines_from(line_length : usize, tiles_by_fingerprints : &HashMap<usize, Vec<Tile>>, line : &mut Vec<Tile>) -> Vec<Vec<Tile>> {
    if line.len() == line_length {
        return vec!(line.clone());
    }
    let end_of_line = &line[line.len() - 1];
    let mut result : Vec<Vec<Tile>> = Vec::new();
    for next_tile in tiles_by_fingerprints.get(&inv10(end_of_line.prints.2)).unwrap_or(&vec!()) {
        if line.iter().find(|existing_tile| existing_tile.id == next_tile.id).is_none() {
            line.push(next_tile.clone());
            result.extend(find_lines_from(line_length, tiles_by_fingerprints, line));
            line.pop();
        }
    }
    return result;
}

fn print_tile(rows: &Vec<Vec<bool>>) {
    for (row_n, row) in rows.iter().enumerate() {
        println!("{}", row.iter().enumerate().map(|(c_n, c)| (if *c { "#".to_string() } else { ".".to_string() }) + (if c_n % 10 == 9 { " " } else { "" })).collect::<String>());
        if row_n % 10 == 9 { println!("\n") };
    }
}

fn main() {
    let mut current_tile_id : usize = 0;
    let mut current_tile : Vec<Vec<bool>> = Vec::new();
    let mut tiles : Vec<(usize, Vec<Vec<bool>>)> = Vec::new();
    let mut tiles_by_fingerprints : HashMap<usize, Vec<Tile>> = HashMap::new();
    let mut tiles_by_number : HashMap<usize, Vec<Tile>> = HashMap::new();
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
    for (tile_number, square) in &tiles {
        let prints = tile_fingerprints(square);
        let tile = Tile { id : *tile_number, prints: prints, rotation: 0, flipped: false };
        for rotation in rotn(tile) {
            tiles_by_fingerprints.entry(rotation.prints.0)
                .or_insert(Vec::new())
                .push(rotation.clone());
            tiles_by_number.entry(*tile_number)
                .or_insert(Vec::new())
                .push(rotation);
        }
        let tile = Tile { id : *tile_number, prints: flip_prints(prints), rotation: 0, flipped: true };
        for rotation in rotn(tile) {
            tiles_by_fingerprints.entry(rotation.prints.0)
                .or_insert(Vec::new())
                .push(rotation.clone());
            tiles_by_number.entry(*tile_number)
                .or_insert(Vec::new())
                .push(rotation);
        }

    }

    let grid_size = (tiles.len() as f32).sqrt() as usize;
    let mut lines : Vec<Vec<Tile>> = Vec::new();
    
    for (tile_number, tiles) in tiles_by_number {
        for tile in tiles {
            lines.extend(find_lines_from(grid_size, &tiles_by_fingerprints, &mut vec!(tile)));
        }
    }
    dbg!(lines.iter().map(|line| line.iter().map(|tile| format!("{}-R{}-F{}", tile.id, tile.rotation, if tile.flipped { 1 } else { 0 })).collect::<Vec<String>>().join(" ")).collect::<Vec<String>>());

    let mut lines_by_starting_cell : HashMap<usize, Vec<&Vec<Tile>>> = HashMap::new();
    for line in &lines {
        lines_by_starting_cell.entry(line[0].id)
            .or_insert(Vec::new())
            .push(line);
    }

    let grids = find_grid(&lines_by_starting_cell);
    if grids.len() == 0 {
        panic!("not found");
    } else {
        let rows = &grids[0];
        let first_row = &rows[0];
        let last_row = &rows[rows.len() - 1];
        dbg!(&first_row[0], &first_row[first_row.len() - 1], &last_row[0], &last_row[last_row.len() - 1]);
        dbg!(first_row[0].id * first_row[first_row.len() - 1].id * last_row[0].id * last_row[last_row.len() - 1].id);
    }

    //                   # 
    // #    ##    ##    ###
    //  #  #  #  #  #  #    
    let sea_monster_template = vec!(vec!(false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true,  false), 
                                    vec!(true,  false, false, false, false, true,  true,  false, false, false, false, true,  true,  false, false, false, false, true,  true,  true), 
                                    vec!(false, true,  false, false, true,  false, false, true,  false, false, true,  false, false, true,  false, false, true,  false, false, false));
 
 
    for rows in grids {
        let mut grid : Vec<Vec<bool>> = Vec::new();
        for row in &rows {
            for extra_row in 0..8 {
                grid.push(Vec::new());
            }
            for cell in row {
                let mut tile_bools : Vec<Vec<bool>> = tiles.iter().find_map(|(id, bools)| if *id == cell.id {
                    Some(bools[1..(bools.len()-1)].iter().map(|boolsrow| boolsrow[1..(boolsrow.len()-1)].to_vec()).collect())
                } else {
                    None
                }).unwrap();
                if cell.flipped {
                    for row in 0..tile_bools.len() {
                        for column in (row+1)..tile_bools[row].len() {
                            let tmp = tile_bools[column][row];
                            tile_bools[column][row] = tile_bools[row][column];
                            tile_bools[row][column] = tmp;
                        }
                    }
                }
                for _ in 0..cell.rotation {
                    tile_bools = (0..tile_bools.len()).map(|n| tile_bools.iter().map(|tile_bools_row| tile_bools_row[n]).rev().collect()).collect();
                }

                let current_top_row = grid.len() - tile_bools.len();
                for (row_number, row) in tile_bools.iter().enumerate() {
                    grid[current_top_row + row_number].extend(row);
                }
            }
        }
        //println!("\n\n");
        dbg!(&rows.iter().map(|cols| cols.iter().map(|tile| format!("{}-R{}-F{}", tile.id, tile.rotation, if tile.flipped { 1 } else { 0 })).collect::<Vec<String>>().join(" ")).collect::<Vec<String>>());
        //print_tile(&grid);
        let mut sea_monster_count = 0;
        for start_row_number in 0..(grid.len() - sea_monster_template.len()) {
            'search: for start_col_number in 0..(grid[0].len() - sea_monster_template[0].len()) {
                for row in 0..sea_monster_template.len() {
                    for col in 0..sea_monster_template[0].len() {
                        if sea_monster_template[row][col] {
                           if !grid[start_row_number + row][start_col_number + col] {
                               continue 'search;
                           }
                        }
                    }
                }
                dbg!(start_row_number, start_col_number);
                sea_monster_count += 1;
            }
        }
        if (sea_monster_count > 0) {
            let sea_monster_cells = sea_monster_template.iter().map(|sea_monster_row| sea_monster_row.iter()).flatten().filter(|t| **t).count();
            let all_visible_cells = grid.iter().map(|grid_row| grid_row.iter()).flatten().filter(|t| **t).count();
            dbg!(sea_monster_cells, sea_monster_count, all_visible_cells, all_visible_cells - sea_monster_count * sea_monster_cells);
        }

    }
}

fn find_grid(lines_by_starting_cell : &HashMap<usize, Vec<&Vec<Tile>>>) -> Vec<Vec<Vec<Tile>>> {
    let mut result = Vec::new();
    for (starting_cell, lines) in lines_by_starting_cell {
        for column in lines.iter() {
            for row in lines.iter() {
                if row.get(1..).unwrap().iter().find(|row_cell| column.iter().find(|column_cell| row_cell.id == column_cell.id).is_some()).is_some() {
                    continue;
                }
                match fill_out_grid(&lines_by_starting_cell, column, &vec!(row.to_vec())) {
                    Some(rows) => {
                        result.push(rows);
                    },
                    None => { /* do nothing */ }
                }
            }
        }
    }
    return result;
}


fn fill_out_grid(lines_by_starting_cell : &HashMap<usize, Vec<&Vec<Tile>>>, column : &Vec<Tile>, rows : &Vec<Vec<Tile>>) -> Option<Vec<Vec<Tile>>> {
    if rows.len() == column.len() {
        return Some(rows.clone());
    }
    let row_number = rows.len();
    let last_row = &rows[row_number - 1];
    'candidates: for candidate in lines_by_starting_cell.get(&column[row_number].id).unwrap() {
        if candidate.iter().find(|candidate_elem| {
            rows.iter().find(|row| {
              row.iter().find(|rows_elem| rows_elem.id == candidate_elem.id).is_some()
            }).is_some()
        }).is_some() {
            continue;
        }
        for (last_row_tile, candidate_row_tile) in last_row.iter().zip(candidate.iter()) {
            if inv10(last_row_tile.prints.3) != candidate_row_tile.prints.1 {
                continue 'candidates;
            }
        }
        let mut new_rows = rows.clone();
        new_rows.push((*candidate).clone());
        match fill_out_grid(lines_by_starting_cell, column, &new_rows) {
            Some(result) => return Some(result),
            _ => { /* do nothing */ }
        }
    }
    return None;
}
