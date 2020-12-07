use std::io::{self, Read};
use std::collections::HashMap;
use std::collections::HashSet;

fn main() {
    let mut input : String = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut num1 = 0;
    let mut num2 = 0;
    for group in input.split("\n\n") {
        let mut num_in_group : usize = 0;
        let mut declared : HashMap<char, HashSet<usize>> = HashMap::new();
        for (member_number, member) in group.split("\n").enumerate() {
            if member.len() == 0 {
                continue;
            }
            for char in member.chars() {
                declared.entry(char).or_insert(HashSet::new()).insert(member_number);
            }
            num_in_group += 1;
        }
        num1 += declared.len();
        num2 += declared.values().filter(|v| v.len() == num_in_group).count();
    }
    println!("{}", num1);
    println!("{}", num2);
    println!("{}", input.split("\n\n").map(|group| {
        let members : Vec<&str> = group.split("\n").filter(|x| x.len() != 0).collect();
        let chars : HashSet<char> = members.iter().map(|x| x.chars()).flatten().collect();
        chars.iter().filter(|c| members.iter().all(|member| member.contains(**c))).count()
    }).sum::<usize>());

}
