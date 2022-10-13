pub fn part_one(input: &str) -> i64 {
    let mut result: i64 = 0;
    for line in input.trim().split("\n\n") {
        let codes:Vec<&str> = line.split_whitespace().collect();
        let mut inc: bool = true;
        if codes.len() < 7 {
            inc = false;
        }
        if codes.len() == 7 {
            for code in codes {
                if code.contains("cid:") {
                    inc = false;
                }
            }
        }
        if inc {
            result += 1;
        }
    }
    println!("day04 -> part one: {}", result);
    return result;
}

pub fn part_two(input: &str) -> i64 {
    return 0;
}

#[cfg(test)]
mod tests {
    #[test]
    fn part_one() {
        assert_eq!(super::part_one(include_str!("testinput")), 2);
    }
    #[test]
    fn part_two() {
        assert_eq!(super::part_two(include_str!("testinput")), 1);
    }
}
