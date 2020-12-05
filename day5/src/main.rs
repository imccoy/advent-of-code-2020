use std::io::{self, BufRead};
use core::cmp::max;

fn main() {
    let mut full = [false;8*128];
    let mut max_seat_id = 0;
    for wrapped_line in io::stdin().lock().lines() {
        let line = wrapped_line.unwrap();
        let mut row_min = 0;
        let mut row_max = 127;
        let mut col_min = 0;
        let mut col_max = 7;
        for c in line.chars() {
            match c {
                'F' => { row_max = row_min + (row_max - row_min + 1) / 2 - 1; }
                'B' => { row_min = row_min + (row_max - row_min + 1) / 2; }
                'L' => { col_max = col_min + (col_max - col_min + 1) / 2 - 1; }
                'R' => { col_min = col_min + (col_max - col_min + 1) / 2; }
                _ => {}
            }
        }
        let seat_id = row_min * 8 + col_min;
        full[seat_id] = true;
        max_seat_id = max(seat_id, max_seat_id);
    }
    println!("{}", max_seat_id);
    for i in (1..(8*127)) {
        if !full[i] && full[i-1] && full[i+1] {
            println!("{}", i);
        }
    }
}
