use std::io::{self, Read};

fn crab_game(mut circle: Vec<usize>, rounds : usize) -> Vec<usize> {
    let original_circle_len = circle.len();

    let mut nexts : [usize; 1000002] = [0; 1000002];
    for i in 0..circle.len() - 1 {
        nexts[circle[i]] = circle[i + 1];
    }
    nexts[circle[original_circle_len - 1]] = circle[0];


    let mut current_cup = circle[0];

    for _ in 0..rounds {
        let mut held_cups = Vec::new();
        let mut next_adjacent_to_current_cup = nexts[current_cup];
        for _ in 0..3 {
            held_cups.push(next_adjacent_to_current_cup);
            next_adjacent_to_current_cup = nexts[next_adjacent_to_current_cup];
        }
        nexts[current_cup] = next_adjacent_to_current_cup;

        let mut destination_label = current_cup;
        loop {
            if destination_label == 1 {
                destination_label = original_circle_len;
            } else {
                destination_label -= 1;
            }
            if held_cups.iter().find(|held_cup| **held_cup == destination_label).is_none() {
                break;
            }
        }

        let after_destination_cup = nexts[destination_label];
        nexts[destination_label] = held_cups[0];
        nexts[held_cups[2]] = after_destination_cup;
        
        current_cup = nexts[current_cup];
    }
    for index in 0..circle.len() {
        circle[index] = current_cup;
        current_cup = nexts[current_cup];
    }
    return circle;
}

fn main() {
    let mut input_string : String = String::new();
    io::stdin().read_to_string(&mut input_string).unwrap();

    let original_input : Vec<usize> = input_string.chars().filter(|c| *c != '\n').map(|c| c.to_string().parse::<usize>().unwrap()).collect();
    println!("{}", crab_game(original_input.clone(), 100).iter().map(|c| c.to_string()).collect::<String>());
    
    let mut expanded_input = original_input.clone();
    for i in (original_input.iter().max().unwrap() + 1)..1000001 {
        expanded_input.push(i);
    }
    for window in crab_game(expanded_input, 10000000).windows(3) {
        if window[0] == 1 {
            dbg!(window, window[1], window[2], window[1] * window[2]);
        }
    }
}
