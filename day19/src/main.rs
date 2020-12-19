use std::io::{self, Read};
use std::collections::HashMap;

#[derive(Debug,Clone)]
enum Rule { Disjunction(Vec<Rule>), Literal(String), Sequence(Vec<usize>) }

fn parse_rule(rule_string : &str) -> Rule {
    if rule_string.contains('|') {
        return Rule::Disjunction(rule_string.split(" | ").map(|s| parse_rule(s)).collect());
    } else if rule_string.contains('"') {
        return Rule::Literal(rule_string[1..(rule_string.len() - 1)].to_string());
    } else {
        return Rule::Sequence(rule_string.split(' ').map(|s| s.parse::<usize>().unwrap()).collect());
    }
}

fn match_rule_number(rules : &HashMap<usize, Rule>, string: &str, rule_number: usize) -> Vec<usize> {
    return match_rule(rules, string, rules.get(&rule_number).unwrap());
}

// for each way in which the rule matches, return the length of the substrings that it matches
fn match_rule(rules : &HashMap<usize, Rule>, string: &str, rule: &Rule) -> Vec<usize> {
    match rule {
        Rule::Disjunction(disjunction_rules) => disjunction_rules.iter().map(|r| match_rule(rules, string, r)).flatten().collect(),
        Rule::Literal(pattern) => if string.starts_with(pattern) { vec!(pattern.len()) } else { vec!() },
        Rule::Sequence(rule_numbers) => {
            let mut offsets = vec!(0);
            for rule_number in rule_numbers {
                let mut new_offsets = Vec::new();
                for offset in offsets {
                    new_offsets.extend(match_rule_number(&rules, &string[offset..], *rule_number).iter().map(|o| offset + o));
                }
                offsets = new_offsets;
            }
            return offsets;
        }
    }
}

fn matches_zero(rules : &HashMap<usize, Rule>, string: &str) -> bool {
   return match_rule_number(rules, string, 0).iter().any(|o| *o == string.len());
}

fn main() {
    let mut input : String = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let sections : Vec<&str> = input.split("\n\n").collect();
    let mut rules : HashMap<usize, Rule> = sections[0].split("\n").map(|rule| {
        let mut iter = rule.split(": ");
        let number = iter.next().unwrap().parse::<usize>().unwrap();
        let body = parse_rule(iter.next().unwrap());
        return (number, body);
   }).collect();
   println!("{}", sections[1].split("\n").filter(|c| matches_zero(&rules, c)).count());

   rules.insert(8, Rule::Disjunction(vec!(Rule::Sequence(vec!(42)), Rule::Sequence(vec!(42, 8)))));
   rules.insert(11, Rule::Disjunction(vec!(Rule::Sequence(vec!(42, 31)), Rule::Sequence(vec!(42, 11, 31)))));

   println!("{}", sections[1].split("\n").filter(|c| matches_zero(&rules, c)).count());
}
