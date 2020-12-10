use std::io::{self, BufRead};

fn count_ways(input_nums : &Vec<u64>) -> u64 {
    let max_jolts = input_nums[input_nums.len() - 1] as usize;
    let mut have_adapter : Vec<bool> = [false].repeat(max_jolts + 1);
    for adapter in input_nums {
        have_adapter[*adapter as usize] = true;
    }
    let mut ways : [u64; 3] = [
      1,
      if have_adapter[1] { 1 } else { 0 },
      if have_adapter[2] { if have_adapter[1] { 2 } else { 1 } } else { 0 }
    ];
    for jolts in 3..(max_jolts+1) {
        let sum = ways[0] + ways[1] + ways[2];
        ways[0] = ways[1];
        ways[1] = ways[2];
        ways[2] = if have_adapter[jolts] { sum } else { 0 };
    }
    ways[2]
}

fn main() {
    let mut nums : Vec<u64> = io::stdin().lock().lines().map(|line| line.unwrap().parse::<u64>().unwrap()).collect();
    nums.push(0);
    nums.sort();
    nums.push(nums.get(nums.len() - 1).unwrap() + 3);
    let mut diffs_1 = 0;
    let mut diffs_3 = 0;
    for i in 0..(nums.len() - 1) {
        let diff = nums[i + 1] - nums[i];
        println!("{} {} {}", nums[i], nums[i+1], diff);
        if diff == 1 {
            diffs_1 += 1;
        } else if diff == 3 {
            diffs_3 += 1;
        }
    }
    println!("{}", diffs_1 * diffs_3);
    println!("{}", count_ways(&nums));
}
