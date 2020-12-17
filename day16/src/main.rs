use core::ops::Range;
use std::io::{self, BufRead};

#[derive(Debug)]
struct Rule { name : String, ranges : Vec<Range<usize>> }

fn parse_rules(iter : &mut std::io::Lines<std::io::StdinLock<'_>>) -> Vec<Rule> {
    let mut rules = Vec::new();
    loop {
        let line = iter.next().unwrap().unwrap();
        if line == "" {
            break;
        }
        let name_and_ranges = line.split(": ").collect::<Vec<&str>>();
        rules.push(Rule { 
            name: name_and_ranges[0].to_string(),
            ranges: name_and_ranges[1].split(" or ").map(|s| {
                let mut iter = s.split("-");
                let start = iter.next();
                let end = iter.next();
                start.unwrap().parse::<usize>().unwrap()..(end.unwrap().parse::<usize>().unwrap() + 1)
            }).collect()
        });
    }
    rules
}

fn parse_your_ticket(iter : &mut std::io::Lines<std::io::StdinLock<'_>>) -> Vec<usize> {
    let _line1 = iter.next().unwrap().unwrap();
    let line2 = iter.next().unwrap().unwrap();
    let _emptyline = iter.next().unwrap().unwrap();
    line2.split(",").map(|s| s.parse::<usize>().unwrap()).collect()
}

fn parse_other_tickets(iter : &mut std::io::Lines<std::io::StdinLock<'_>>) -> Vec<Vec<usize>> {
    let _line1 = iter.next().unwrap().unwrap();
    iter.map(|line| line.unwrap().split(",").map(|s| s.parse::<usize>().unwrap()).collect()).collect()
}

fn valid_for_at_least_one_field(rules : &Vec<Rule>, n: usize) -> bool {
    for rule in rules {
        for range in &rule.ranges {
            if range.contains(&n) {
                return true;
            }
        }
    }
    return false;
}

fn part1(rules : &Vec<Rule>, tickets : &Vec<Vec<usize>>) -> usize {
    tickets.iter()
      .map(|ticket| ticket.iter()
                      .map(|num| if !valid_for_at_least_one_field(&rules, *num) { *num } else { 0 })
                      .sum::<usize>())
      .sum()
}

fn main() {
    let stdin = io::stdin();
    let iter_lock = stdin.lock();
    let mut lines = iter_lock.lines();
    let rules = parse_rules(&mut lines);
    let your_ticket = parse_your_ticket(&mut lines);
    let other_tickets = parse_other_tickets(&mut lines);
    let num_columns = your_ticket.len();
    println!("part 1: {}", part1(&rules, &other_tickets));

    let valid_tickets = other_tickets.iter()
                          .filter(|ticket| ticket.iter().all(|n| valid_for_at_least_one_field(&rules, *n)))
                          .collect::<Vec<&Vec<usize>>>();

    let mut possibles : Vec<Vec<String>> = Vec::new();
    possibles.resize_with(rules.len(), || Vec::new());
    for rule in rules.iter() {
        for column in 0..num_columns {
            let all_valid = valid_tickets.iter().all(|ticket| {
                for range in &rule.ranges {
                    if range.contains(&ticket[column]) {
                        return true;
                    }
                }
                return false;
            });
            if all_valid {
                possibles[column].push(rule.name.to_string());
            }
        }
    }


    let mut assignments : Vec<(usize, String)> = Vec::new();
    while assignments.len() != num_columns {
        let fields : Vec<usize> = possibles.iter().enumerate().filter_map(|(index,possible)| if possible.len() == 1 { Some(index) } else { None }).collect();
        if fields.len() != 1 {
            println!("E_NOT_SOPHISTICATED");
        }
        let field_name = possibles[fields[0]][0].to_string();
        assignments.push((fields[0], field_name.to_string()));
        for possible in &mut possibles {
            possible.retain(|f| field_name != *f);
        }
    }
    
    println!("part 2: {}", assignments
                             .iter()
                             .filter_map(|(index, field_name)| if field_name.starts_with("departure") { Some(index) } else { None })
                             .map(|index| your_ticket[*index])
                             .product::<usize>());
}
