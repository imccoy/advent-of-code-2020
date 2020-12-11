use std::io::{self, BufRead};

fn is_sum_of_two_numbers_in_previous_n(nums: &Vec<u64>, start : usize, end : usize) -> bool {
    for i in start..end {
        for j in i+1..end {
            if nums[i] + nums[j] == nums[end] {
                return true;
            }
        }
    }
    return false;
}

fn find_not_sum_of_two_numbers_in_previous_n(nums: &Vec<u64>) -> Option<u64> {
    for start in 0..(nums.len()-25) {
        let end = start + 25;
        if !is_sum_of_two_numbers_in_previous_n(nums, start, end) {
            return Some(nums[end]);
        }
    }
    return None;
}


fn main() {
    let nums : Vec<u64> = io::stdin().lock().lines().map(|line| line.unwrap().parse::<u64>().unwrap()).collect();
    let special_number = find_not_sum_of_two_numbers_in_previous_n(&nums).unwrap();
    println!("{}", special_number);
    for i in 0..nums.len() {
        let mut sum = nums[i];
        for j in i+1..nums.len() {
            sum += nums[j];
            if sum > special_number {
                break;
            }
            if sum == special_number {
                let range = nums.get(i..j).unwrap();
                let min = range.iter().min().unwrap();
                let max = range.iter().max().unwrap();
                dbg!(min, max, min + max);
            }
        }
    }

}
