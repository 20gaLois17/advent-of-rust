use std::collections::HashSet;
use std::collections::HashMap;

pub fn part_one(input: &str) -> i64 {
    let mut all_bags: Vec<Bag> = vec![];
    let mut found_colors: HashSet<String> = HashSet::new();

    for line in input.trim().split('\n') {
        all_bags.push(Bag::new(line));
    }
    // init: we are starting with "shiny gold" bag
    let mut candidates: Vec<Bag> = vec![
        Bag {
            color: "shiny gold".to_string(),
            nested_colors: HashMap::from_iter([("dark red".to_string(), 2)])
        }
    ];

    // go down the 'can_contain - chain' until there is no bag left
    // we are implicitly assuming that there is no circular reference in the chain (-> inf loop)
    while let Some(bag) = candidates.pop() {
        for parent in &all_bags {
            if parent.can_contain(&bag.color) {
                found_colors.insert(parent.color.to_string());
                candidates.push(parent.clone());
            }
        }
    }
    let result = found_colors.len().try_into().unwrap();

    println!("day07 -> part one: {}", result);
    return result;
}

pub fn part_two(input: &str) -> i64 {
    let mut all_bags: Vec<Bag> = vec![];
    for line in input.trim().split('\n') {
        all_bags.push(Bag::new(line));
    }
    let list: Vec<&Bag>   = all_bags.iter().filter(|e| e.color == "shiny gold".to_string()).collect();
    assert_eq!(list.len(), 1);
    let shiny_gold: Bag   = list[0].clone();

    let result: i64 = count_nested(shiny_gold, all_bags).try_into().unwrap();
    println!("day07 -> part two: {}", result);
    return result;
}

#[derive(Eq, PartialEq, Debug, Clone)]
struct Bag {
    color: String,
    nested_colors: HashMap<String, usize>
}

impl Bag {
    fn new(description: &str) -> Self {
        let s: Vec<&str> = description.split("contain").collect();
        assert_eq!(s.len(), 2);

        let self_color = s[0].replace(" bags", "").trim().to_string();
        let mut self_quantities = HashMap::new();

        for desc in s[1].split(',') {
            if desc.trim() == "no other bags." {
                break;
            } else {
                let chunk: Vec<String> = desc.split(' ').map(|e| e.to_string()).collect();
                let col: String = chunk[2..4].join(" ").to_string();
                self_quantities.insert(col, chunk[1].parse::<usize>().unwrap());
           }
        }

        return Self {
            color: self_color,
            nested_colors: self_quantities
        }
    }
    fn can_contain(&self, color: &str) -> bool {
        return self.nested_colors.contains_key(&color.to_string());
    }
}

// recursive helper function
// there don't seem to be circular relations ...
fn count_nested(bag: Bag, all_bags: Vec<Bag>) -> usize {
    let mut next_round_candidates: Vec<Bag> = vec![];
    for child in &all_bags {
        if bag.can_contain(&child.color) {
            next_round_candidates.push(child.clone());
        }
    }
    if next_round_candidates.len() == 0 {
        return 0;
    } else {
        return next_round_candidates.iter().map(|e| {
                let multiplier: usize = bag.nested_colors[&e.color];
                return multiplier * (1 + count_nested(e.clone(), all_bags.clone()))
            }).sum()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn part_one() {
        assert_eq!(super::part_one(include_str!("testinput")), 4);
    }
    #[test]
    fn can_contain() {
        assert!(super::Bag::new("solid black bags contain 1 bright white bag")
                .can_contain("bright white"));
        assert!(super::Bag::new("solid black bags contain 1 bright dark blue, 2 mellow green bags.")
                .can_contain("mellow green"));
        assert!(!super::Bag::new("solid black bags contain 1 bright dark blue, 2 mellow green bags.")
                .can_contain("light blue"));
    }
    #[test]
    fn part_two() {
        assert_eq!(super::part_two(include_str!("testinput")), 32);
        assert_eq!(super::part_two(include_str!("testinput_2")), 126);
    }
}
