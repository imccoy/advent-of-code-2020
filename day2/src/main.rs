use std::io::{self, BufRead};
use std::convert::TryInto;
use regex::Regex;

fn main() {
    let mut valid_count_a = 0;
    let mut valid_count_b = 0;
    let re = Regex::new(r"^(\d*)-(\d*) ([a-z]): ([a-z]*)$").unwrap();
    for wrapped_line in io::stdin().lock().lines() {
        let line = wrapped_line.unwrap();
        let cap = re.captures(&line).unwrap();
        let min = cap[1].parse::<i32>().unwrap();
        let max = cap[2].parse::<i32>().unwrap();
        let search_char = cap[3].chars().nth(0).unwrap();
        let pass = &cap[4];

        let mut char_count = 0;
        for current_char in pass.chars() {
            if search_char == current_char {
                char_count += 1;
            }
        }
        if char_count >= min && char_count <= max {
            valid_count_a += 1;
        }
        let p1 = pass.chars().nth((min - 1).try_into().unwrap()).unwrap() == search_char;
        let p2 = pass.chars().nth((max - 1).try_into().unwrap()).unwrap() == search_char;
        if (p1 || p2) && !(p1 && p2) {
            valid_count_b += 1;
        }
    }
    println!("{} {}", valid_count_a, valid_count_b);
}
