use std::io::{self, BufRead};

fn main() {
    let lines : Vec<String> = io::stdin().lock().lines().map(|l| l.unwrap()).collect();
    let ready_at = lines[0].parse::<usize>().unwrap();
    let possibly_times : Vec<&str> = lines[1].split(',').collect();

    let known_times = possibly_times.iter()
      .filter(|t| **t != "x")
      .map(|t| t.parse::<usize>().unwrap());

    let (_, shortest_wait_bus) = known_times.map(|t| (ready_at % t, t)).max().unwrap();

    let shortest_wait_bus_arrives_at = (ready_at + shortest_wait_bus - 1) / shortest_wait_bus * shortest_wait_bus;
    println!("{}, {}, {}", ready_at, shortest_wait_bus_arrives_at, shortest_wait_bus);
    println!("{}", (shortest_wait_bus_arrives_at - ready_at) * shortest_wait_bus);

    let offset_factors : Vec<(u128, u128)> = possibly_times.iter().enumerate()
      .filter(|(_, factor)| **factor != "x")
      .map(|(offset, factor)| (offset as u128, factor.parse::<u128>().unwrap()))
      .collect();

    let mut decreasing_factor_offset_factors = offset_factors.clone();
    decreasing_factor_offset_factors.sort_by(|(_, factor1), (_, factor2)| factor2.partial_cmp(factor1).unwrap());

    let biggest_factor_offset : u128 = decreasing_factor_offset_factors[0].0;
    let biggest_factor : u128 = decreasing_factor_offset_factors[0].1;

    let mut start = 0;
    let mut jump = 1;
    let mut lastn = 0;
    while lastn < decreasing_factor_offset_factors.len() {
        let mut i = start;
        let mut n0 = 0;
        loop {
            let mut all = true;
            for (offset, factor) in &decreasing_factor_offset_factors[0..(lastn+1)] {
               if (i + offset) % factor != 0 {
                   all = false;
               } 
            }
            if all {
                if n0 == 0 {
                    n0 = i;
                } else {
                    start = n0;
                    jump = i - n0;
                    break;
                }
            }
            i += jump;
        }
        lastn += 1;
    }
    println!("{}", start);
}

fn n_with_no_distance_from(multiplier: u128, offset: u128, factor: u128, start_point: u128) -> u128 {
    let mut n = 1;
    loop {
        if (start_point + (n * multiplier) + offset) % factor == 0 {
            return n;
        }
        n += 1;
    }
}

/*
[src/main.rs:39] differences = [
    320,
    13,
    11,
    26,
    15,
    10,
    3,
    10,
]
*/
