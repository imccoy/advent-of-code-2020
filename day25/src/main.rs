use std::io::{self, BufRead};

fn transforms(subject_number : u64) -> impl Iterator<Item = (usize, u64)> {
    let mut value : u64 = 1;
    (1..).map(move |n| {
        value = (value * subject_number) % 20201227;
        (n, value)
    })
}

fn num_transforms_from(subject_number : u64, target : u64) -> Option<usize> {
    transforms(subject_number).find_map(|(n, v)| if v == target { Some(n) } else { None })
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    let card_public_key = lines.next().unwrap().unwrap().parse::<u64>().unwrap();
    let door_public_key = lines.next().unwrap().unwrap().parse::<u64>().unwrap();
    
    dbg!(card_public_key, door_public_key);

    let card_iterations = num_transforms_from(7, card_public_key).unwrap();
    let door_iterations = num_transforms_from(7, door_public_key).unwrap();
    dbg!(card_iterations, door_iterations);

    let card_calculated_key = transforms(door_public_key).nth(card_iterations - 1).unwrap().1; 
    let door_calculated_key = transforms(card_public_key).nth(door_iterations - 1).unwrap().1; 
    dbg!(card_calculated_key, door_calculated_key);
}
