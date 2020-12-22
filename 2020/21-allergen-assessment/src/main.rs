use regex::Regex;
use std::collections::{HashMap, HashSet};

/// Returns mapping from allergens to candidate ingredients ('fish' -> {'foo', 'bar', 'baz'}
fn parse(input: &str) -> (HashMap<String, HashSet<String>>, HashMap<String, u32>) {
    let re = Regex::new(r"^((\w+ )+)\(contains ((\w+)(, \w+)*)\)$").unwrap();
    let mut mapping = HashMap::<String, HashSet<String>>::new();
    let mut counts = HashMap::<String, u32>::new();
    for line_cap in input.lines().map(|l| re.captures(l).unwrap()) {
        let ingredients = line_cap[1].split_whitespace();
        let allergents = line_cap[3].split(',').map(str::trim);

        for ingredient in ingredients.clone() {
            let entry = counts.entry(ingredient.to_string()).or_insert(0);
            *entry += 1;
        }
        for allergent in allergents {
            let entry = mapping
                .entry(allergent.to_string())
                .or_insert_with(HashSet::new);
            if !entry.is_empty() {
                let current = entry.clone();
                let other = ingredients
                    .clone()
                    .map(String::from)
                    .collect::<HashSet<String>>();
                *entry = current
                    .intersection(&other)
                    .cloned()
                    .collect::<HashSet<String>>();
            } else {
                for ingredient in ingredients.clone() {
                    entry.insert(ingredient.to_string());
                }
            }
        }
    }
    (mapping, counts)
}

fn solve(input: &str) -> u32 {
    let (mut allergents, counts) = parse(input);
    println!("mapping: {:?}", allergents);

    let mut known = HashSet::<String>::new();
    while known.len() != allergents.len() {
        for ingredients in allergents.values_mut() {
            if ingredients.len() > 1 {
                for k in &known {
                    ingredients.remove(k);
                }
            } else {
                for ingredient in ingredients.iter() {
                    known.insert(ingredient.to_string());
                }
            }
        }
    }

    let nasties = allergents
        .values()
        .map(|m| m.iter().next().unwrap().clone())
        .collect::<HashSet<String>>();
    println!("nasties: {:?}", nasties);
    counts
        .iter()
        .filter_map(|(k, v)| if nasties.contains(k) { None } else { Some(v) })
        .sum()
    // TODO summarize counts for all values that hasn't been mapped to
}

fn main() {
    let input = include_str!("input");
    println!("{}", solve(input));
}

#[test]
fn test() {
    let input = include_str!("input-simple");
    assert_eq!(solve(input), 5);
}
