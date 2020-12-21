extern crate regex;

use regex::Regex;
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::BTreeMap;
use std::iter::FromIterator;

pub fn part1(inp: String) {
    let input = read_input(&inp);
    let mut map = parse_input(&input);

    let known_allergens = get_known_allergens(&mut map);

    let sum: usize = input
        .iter()
        .map(|(ingredients, _)| ingredients
            .iter()
            .filter(|i| !known_allergens.contains_key(*i))
            .count()
        ).sum();

    println!("sum: {:?}", sum);
}

pub fn part2(inp: String) {
    let input = read_input(&inp);
    let mut map = parse_input(&input);

    let known_allergens = get_known_allergens(&mut map);
    
    let flipped = known_allergens.iter().map(|(a, b)| (b.to_string(),a.to_string())).collect::<BTreeMap<String, String>>();
    let result: Vec<String> = flipped.values().cloned().collect();

    println!("result: {:?}", result.join(","));
}

fn get_known_allergens(map: &mut HashMap<String, Vec<HashSet<String>>>) -> HashMap<String, String> {
    let mut known_allergens: HashMap<String, String> = HashMap::new();

    loop {
        let (allergen, ingredient) = match find_known_allergen(&map) {
            Some(tup) => tup,
            None => return known_allergens,
        };

        remove_known_ingredient(map, &ingredient);
        known_allergens.insert(ingredient, allergen);
    }
}

fn find_known_allergen(map: &HashMap<String, Vec<HashSet<String>>>) -> Option<(String, String)> {
    for (allergen, ingredient_list_list) in map {
        let mut intersected_list = ingredient_list_list[0].clone();
        
        for ingredient_list in ingredient_list_list {
            intersected_list = intersected_list.intersection(&ingredient_list).cloned().collect();
        }
        
        if (intersected_list.len() == 1) {
            let known_ingredient = intersected_list.iter().next().unwrap();
            return Some((allergen.clone(), known_ingredient.clone()));
        }
        
    }
    None
    
}

fn remove_known_ingredient(map: &mut HashMap<String, Vec<HashSet<String>>>, ingredient: &String) {
    for (allergen, ingredient_list_list) in map.iter_mut() {
        for ingredient_list in ingredient_list_list.iter_mut() {
            ingredient_list.remove(ingredient);
        }
    }
}

fn read_input(inp: &str) -> Vec<(Vec<String>, Vec<String>)> {
    let mut result: Vec<(Vec<String>, Vec<String>)> = vec![];

    let lines: Vec<&str> = inp.split("\n").filter(|line| line.len() > 0).collect();

    let regex = Regex::new(r"^(.+) \(contains (.+)\)$").unwrap();

    for line in lines {
        let captures = match regex.captures(line) {
            Some(captures) => captures,
            None => panic!("did not match regex: {}", line),
        };

        let ingredients: Vec<String> = captures[1].split(" ").map(str::to_string).collect();
        let allergens: Vec<String> = captures[2].split(", ").map(str::to_string).collect();

        result.push((ingredients, allergens));
    }

    result
}
fn parse_input(input: &Vec<(Vec<String>, Vec<String>)>) -> HashMap<String, Vec<HashSet<String>>> {
    let mut map: HashMap<String, Vec<HashSet<String>>> = HashMap::new();

    for (ingredients, allergens) in input {
        for allergen in allergens {
            map.entry(allergen.to_string()).or_insert_with(Vec::new).push(HashSet::from_iter(ingredients.iter().cloned()));
        }
    }

    map
}