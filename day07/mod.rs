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
    // for bag in &all_bags {
    //     println!("{:?}", bag);
    // }
    // init: we are starting with "shiny gold" bag
    let mut candidates: Vec<Bag> = vec![
        Bag {
            color: "shiny gold".to_string(), 
            nested_colors: HashMap::from_iter([("dark red".to_string(), 2)])
        }
    ];

    let mut result: i64 = 0;
    while let Some(bag) = candidates.pop() {
        for child in &all_bags {
            if bag.can_contain(&child.color) {
                candidates.push(child.clone());
            }
        }
    }
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
                self_quantities.insert(col.clone(), chunk[1].parse::<usize>().unwrap());
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
        assert!(super::Bag::new("shiny gold bags contain 2 dark red bags.")
                .can_contain("dark red"));
    }
    #[test]
    fn part_two() {
        assert_eq!(super::part_two(include_str!("testinput_2")), 126);
    }
}
