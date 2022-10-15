use std::collections::HashSet;

pub fn part_one(input: &str) -> i64 {
    let mut result: usize = 0;
    for group in input.trim().split("\n\n") {
        let mut answers: HashSet<char> = HashSet::new();
        for person in group.split("\n") {
            for ans in person.chars() {
                answers.insert(ans);
            }
        }
        result += answers.len();
    }
    println!("day06 -> part one: {}", result);
    return result.try_into().unwrap();
}
pub fn part_two(input: &str) -> i64 {
    let mut result: usize = 0;
    for group in input.trim().split("\n\n") {
        let persons: Vec<&str> = group.split("\n").collect();
        let mut inter: HashSet<char> = HashSet::new();
        for i in 0..persons.len() {
            if i == 0 {
                // create initial hash map
                inter = HashSet::from_iter(persons[i].chars());
            } else {
                // compute intersection between person's answers
                inter = inter.intersection(&HashSet::from_iter(persons[i].chars())).copied().collect();
            }
        }
        result += inter.len();
    }
    println!("day06 -> part two: {}", result);
    return result.try_into().unwrap();
}
#[cfg(test)]
mod tests {
    #[test]
    fn part_one() {
        assert_eq!(super::part_one(include_str!("testinput")), 11);
    }
    #[test]
    fn part_two() {
        assert_eq!(super::part_two(include_str!("testinput")), 6);
    }
}
