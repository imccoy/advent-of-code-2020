use std::io::{self, BufRead};
use std::collections::HashMap;
use std::collections::HashSet;

fn split_once(string: &str, delim: &str) -> (String, String) {
    let (a, b) = string.split_at(string.find(delim).unwrap());
    (a.to_string(), b.get(delim.len()..).unwrap().to_string())
}

fn count_ways_of_containing(start_bag : String, bags_contained_by : HashMap<String, HashSet<String>>) -> usize {
    let mut ways : HashSet<String> = HashSet::new();
    let mut new_ways : Vec<String> = Vec::new();
    new_ways.push(start_bag);
    while let Some(new_way) = new_ways.pop() {
        ways.insert(new_way.to_string());
        if let Some(reachable_bags) = bags_contained_by.get(&new_way) {
            for reachable_bag in reachable_bags {
                if !ways.contains(reachable_bag) {
                    new_ways.push(reachable_bag.to_string());
                }
            }
        }
    }

    ways.len() - 1
}

fn count_bags_in(start_bag : String, bags_contain : HashMap<String, HashMap<String, u32>>) -> u32 {
    let mut stack : Vec<(String, u32)> = Vec::new();
    let mut bags : u32 = 0;
    stack.push((start_bag, 1));
    while let Some((current_bag, current_bag_count)) = stack.pop() {
        bags += current_bag_count;
        for successors in bags_contain.get(&current_bag) {
            for (successor_bag, successor_bag_count) in successors {
                stack.push((successor_bag.to_string(), successor_bag_count * current_bag_count));
            }
        }
    }
    bags - 1 
}

fn main() {
    let mut bags_contain : HashMap<String, HashMap<String, u32>> = HashMap::new();
    let mut bags_contained_by : HashMap<String, HashSet<String>> = HashMap::new();
    for wrapped_line in io::stdin().lock().lines() {
        let line = wrapped_line.unwrap();
        let (holder, contained_words) = split_once(&line, " bags contain ");
        if contained_words == "no other bags." {
            continue;
        }
        for contained in contained_words.split(", ") {
            let (num, bagbag) = split_once(contained.strip_suffix(".").unwrap_or(&contained), " ");
            let bag = bagbag.strip_suffix(" bag").or(bagbag.strip_suffix(" bags")).unwrap_or(&bagbag);
            bags_contained_by.entry(bag.to_string()).or_insert(HashSet::new())
              .insert(holder.to_string());
            bags_contain.entry(holder.to_string()).or_insert(HashMap::new())
              .insert(bag.to_string(), num.parse::<u32>().unwrap());
        }
    }

    println!("{}", count_ways_of_containing("shiny gold".to_string(), bags_contained_by));
    println!("{}", count_bags_in("shiny gold".to_string(), bags_contain));
}
