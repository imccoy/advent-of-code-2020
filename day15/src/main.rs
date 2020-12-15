use std::collections::HashMap;
use std::io::{self, BufRead};

fn main() {
    let line = io::stdin().lock().lines().next().unwrap().unwrap();
    let initial_numbers = line.split(',').map(|n| n.parse::<usize>().unwrap()).collect::<Vec<usize>>();
    let mut last_appearance : HashMap<usize, usize> = HashMap::with_capacity(3611714 * 5);
    let mut previous_appearance_last_num = 0;
    let mut last_num = 0;
    for (initial_number_index, initial_number) in initial_numbers.iter().enumerate() {
        println!("Turn {}: {}", initial_number_index + 1, initial_number);
        last_appearance.insert(*initial_number, initial_number_index + 1);
        last_num = *initial_number;
    }
    let mut round : usize = initial_numbers.len();
    while round <= 30000000 {
        let next_num = match last_appearance.get(&last_num) {
            Some(n) => round - n,
            None => 0
        };
        if round == 2019 || (round + 1) % 100000 == 0 {
            println!("Turn {}: {}    ({})", round + 1, next_num, last_appearance.len());
        }
        last_appearance.insert(last_num, round);
        
        last_num = next_num;
        round += 1;
    }
    println!("{}", line);
}
