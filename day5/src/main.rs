use std::io::{self, BufRead};
use core::cmp::max;

fn main() {
    let mut full = [false;8*128];
    let mut max_seat_id = 0;
    for wrapped_line in io::stdin().lock().lines() {
        let line = wrapped_line.unwrap();
        let mut seat_id : usize = 0;
        for c in line.chars() {
            seat_id = (seat_id << 1) + (if c == 'B' || c == 'R' { 1 } else { 0 })
        }
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
