use std::collections::HashMap;
use std::io::{self, BufRead};
use regex::Regex;

fn main() {
    let re = Regex::new(r"^([a-z ]*) \(contains ([a-z, ]*)\)$").unwrap();
    let mut foods : Vec<(Vec<String>, Vec<String>)> = Vec::new();
    let mut all_allergens : HashMap<String, Vec<(Vec<String>, Vec<String>)>> = HashMap::new();
    for wrapped_line in io::stdin().lock().lines() {
        let line = wrapped_line.unwrap();
        let cap = re.captures(&line).unwrap();
        let ingredients = cap[1].split(" ").map(|s| s.to_string()).collect::<Vec<_>>();
        let allergens = cap[2].split(", ").map(|s| s.to_string()).collect::<Vec<_>>();
        foods.push((ingredients.clone(), allergens.clone()));
        for allergen in &allergens {
            all_allergens.entry(allergen.to_string())
                .or_insert(Vec::new())
                .push((ingredients.clone(), allergens.clone()));
        }
    }

    println!("{}", all_allergens.len());

    let mut all_possibilities : HashMap<String, Vec<String>> = HashMap::new();
    for (allergen, foods) in all_allergens {
        dbg!(&allergen);
        let mut possibilities = foods[0].0.clone();
        for (ingredients, _) in foods {
            dbg!(&ingredients);
            let mut new_possibilities = Vec::new();
            for possibility in possibilities {
                if ingredients.contains(&possibility) {
                    new_possibilities.push(possibility);
                }
            }
            possibilities = new_possibilities;
            dbg!(&possibilities);
        }
        all_possibilities.insert(allergen.to_string(), possibilities);
    }
    dbg!(&all_possibilities);

    let could_possibly : Vec<String> = all_possibilities.values().flatten().map(|s| s.to_string()).collect();
    let mut could_not_possibly_count = 0;
    for (ingredients, _) in foods {
        for ingredient in ingredients {
            if !could_possibly.contains(&ingredient) {
                could_not_possibly_count += 1;
            }
        }
    }
    dbg!(could_not_possibly_count);

    let mut confirmed_allergens : HashMap<String, String> = HashMap::new();
    let mut confirmed_ingredients : HashMap<String, String> = HashMap::new();
    while confirmed_allergens.len() != all_possibilities.len() {
        for (allergen, ingredients) in &all_possibilities {
            if confirmed_allergens.contains_key(allergen) {
                continue;
            }
            let mut ingredients = ingredients.clone();
            ingredients.retain(|ingredient| !confirmed_ingredients.contains_key(ingredient));
            if ingredients.len() == 1 {
                confirmed_ingredients.insert(ingredients[0].to_string(), allergen.to_string());
                confirmed_allergens.insert(allergen.to_string(), ingredients[0].to_string());
            }
        }
    }

    let mut sorted_allergens : Vec<&String> = confirmed_allergens.keys().collect();
    sorted_allergens.sort();
    println!("{}", sorted_allergens.iter().map(|a| confirmed_allergens.get(*a).unwrap().to_string()).collect::<Vec<_>>().join(","));
}
