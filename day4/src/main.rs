use std::io::{self, BufRead};
use std::collections::HashMap;
use regex::Regex;

fn year_in_range(year_string: &str, min: i32, max: i32) -> bool {
    let year : i32 = year_string.parse().unwrap();
    year >= min && year <= max
}

fn is_height(height_string: &str) -> bool {
    if let Some(metric_string) = height_string.strip_suffix("cm") {
        let metric : i32 = metric_string.parse().unwrap();
        metric >= 150 && metric <= 193
    } else if let Some(imperial_string) = height_string.strip_suffix("in") {
        let imperial : i32 = imperial_string.parse().unwrap();
        imperial >= 59 && imperial <= 76 
    } else {
        false
    }
}


fn is_color(color: &str) -> bool {
    let color_regex : regex::Regex = Regex::new(r"^#[0-9a-f]{6}$").unwrap();
    color_regex.is_match(color)
}

fn main() {
    let mut passports : Vec<HashMap<String,String>> = Vec::new();

    for wrapped_line in io::stdin().lock().lines() {
        let line = wrapped_line.unwrap();
        if line == "" {
            passports.push(HashMap::new());
        } else {
            let mut passport = passports.pop().unwrap_or(HashMap::new());
            for kv in line.split_whitespace() {
              let k = kv.split(':').nth(0).unwrap();
              let v = kv.split(':').nth(1).unwrap();
              passport.insert(k.to_string(), v.to_string());
            }
            println!("{:?}", passport);
            passports.push(passport);
        }
    }

    let mut simple_valid = 0;
    let mut valid = 0;
    for passport in passports {
        if passport.contains_key("byr") &&
            passport.contains_key("iyr") &&
            passport.contains_key("eyr") &&
            passport.contains_key("hgt") &&
            passport.contains_key("hcl") &&
            passport.contains_key("ecl") &&
            passport.contains_key("pid")
        {
            simple_valid += 1;
        }

        if passport.get("byr").map(|val| year_in_range(val, 1920, 2002)).unwrap_or(false) &&
            passport.get("iyr").map(|val| year_in_range(val, 2010, 2020)).unwrap_or(false) &&
            passport.get("eyr").map(|val| year_in_range(val, 2020, 2030)).unwrap_or(false) &&
            passport.get("hgt").map(|val| is_height(val)).unwrap_or(false) &&
            passport.get("hcl").map(|val| is_color(val)).unwrap_or(false) &&
            passport.get("ecl").map(|val| val == "amb" || val == "blu" || val == "brn" || val == "gry" || val == "grn" || val == "hzl" || val == "oth").unwrap_or(false) &&
            passport.get("pid").map(|val| val.len() == 9 && val.chars().all(|c| c.is_digit(10) )).unwrap_or(false)
        {
            valid += 1;
        }
    }

    println!("{} {}", simple_valid, valid);
}
