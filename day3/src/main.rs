use std::io::{self, BufRead};

fn main() {
    let mut positions = [0, 0, 0, 0, 0];
    let mut trees = [0, 0, 0, 0, 0];
    let slopes = [(1,1), (3,1), (5,1), (7,1), (1, 2)];
    for (line_num, wrapped_line) in io::stdin().lock().lines().enumerate() {
        let line = wrapped_line.unwrap();
        for (slope, (x, y)) in slopes.iter().enumerate() {
          if line_num % y != 0 {
            continue;
          }
          let position = &mut positions[slope];
          let trees = &mut trees[slope];

          let char = line.chars().nth(*position % line.len());

          *trees += if char.unwrap() == '#' { 1  } else { 0 };
          *position += x;
        }
    }
    println!("{}", trees[1]);
    println!("{}", trees.iter().product::<u32>());
}
