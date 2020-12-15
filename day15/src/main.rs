use std::io::{self, BufRead};

fn main() {
    let line = io::stdin().lock().lines().next().unwrap().unwrap();
    let initial_numbers = line.split(',').map(|n| n.parse::<usize>().unwrap()).collect::<Vec<usize>>();
    let mut last_appearance : Vec<usize> = Vec::new();
    last_appearance.resize_with(300000000, || 0);
    let mut last_num = 0;
    for (initial_number_index, initial_number) in initial_numbers.iter().enumerate() {
        println!("Turn {}: {}", initial_number_index + 1, initial_number);
        last_appearance[*initial_number] = initial_number_index + 1;
        last_num = *initial_number;
    }
    let mut round : usize = initial_numbers.len();
    while round <= 30000000 {
        let next_num = if last_appearance[last_num] == 0 { 0 } else { round - last_appearance[last_num] };
        if (round + 1) % 100000 == 0 {
            println!("Turn {}: {}", round + 1, next_num);
        }
        last_appearance[last_num] = round;
        
        last_num = next_num;
        round += 1;
    }
    println!("{}", line);
}
