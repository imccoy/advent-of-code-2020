use std::io::{self, BufRead};

fn count_ways(input_nums : &Vec<u64>) -> u64 {
    let max_jolts = input_nums[input_nums.len() - 1] as usize;
    let mut have_adapter : Vec<bool> = [false].repeat(max_jolts + 1);
    for adapter in input_nums {
        have_adapter[*adapter as usize] = true;
    }
    let mut ways : Vec<u64> = Vec::with_capacity(max_jolts + 1);
    ways.push(1); // there's one way to get 0 jolts: plug in to the wall
    for jolts in 1..(max_jolts+1) {
        if have_adapter[jolts] {
            let min = std::cmp::max(0, jolts as i32 - 3) as usize; // we can adapt from jolts - 3 to jolts, as long as jolts - 3 is non-negative
            let max = jolts - 1;
            let accessible_jolts_range = min..(max+1);
            let ways_this_jolt = ways.get(accessible_jolts_range).unwrap().iter().sum();
            ways.push(ways_this_jolt);
        } else {
            ways.push(0);
        }
    }
    ways[max_jolts]
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
