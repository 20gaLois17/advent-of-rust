pub fn part_one(input: &str) -> usize {
    let mut numbers: Vec<usize> = input.trim().split('\n').map(|e| e.parse().unwrap()).collect();
    numbers.sort();
    // there is always one difference of jolt '1' and '3' to account for in advance
    let mut delta_1 = 1;
    let mut delta_3 = 1;
    for i in 0..numbers.len()-1 {
        match numbers[i+1] - numbers[i] {
            1 => delta_1 += 1,
            3 => delta_3 += 1,
            _ => panic!("not a valid delta")
        }
    }
    let result = delta_1 * delta_3;
    println!("day10 -> part one {}", result);
    return result;
}


pub fn part_two(input: &str) -> usize {
    return 0;
}

#[cfg(test)]
mod tests {
    #[test]
    fn part_one() {
        assert_eq!(super::part_one(include_str!("testinput")), 220);
    }
    #[test]
    fn part_two() {
        assert_eq!(super::part_two(include_str!("testinput")), 62);
    }
}
