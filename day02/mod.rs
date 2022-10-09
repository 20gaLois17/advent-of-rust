pub fn part_one(input: &str) -> i64 {
    let mut count_valid: i64 = 0;

    for line in input.trim().split('\n') {
        let input: Vec<&str> = line.split(" ").collect();
        let needle: String   = input[1].replace(":", "");
        let bounds: Vec<i32> = input[0].split("-").map(|x| x.parse::<i32>().unwrap()).collect();

        let matches: i32 = input[2].matches(&needle).count().try_into().unwrap();
        if bounds[0] <= matches && matches <= bounds[1] {
            count_valid += 1;
        }
    }
    println!("day02 -> part one: {}", count_valid);
    return count_valid;
}

pub fn part_two(input: &str) -> i64 {
    let mut count_valid: i64 = 0;

    for line in input.trim().split('\n') {
        let input: Vec<&str> = line.split(" ").collect();
        let needle: String   = input[1].replace(":", "");
        let bounds: Vec<i32> = input[0].split("-").map(|x| x.parse::<i32>().unwrap()).collect();

        let mut b:bool = false;
        for (i, c) in input[2].chars().enumerate() {
            if i > bounds[1].try_into().unwrap() {
                continue;
            }
            if i+1 == bounds[0].try_into().unwrap() || i+1 == bounds[1].try_into().unwrap() {
                if needle == c.to_string() {
                    b = !b;
                }
            }
        }
        if b {
            count_valid += 1;
        }
    }
    println!("day02 -> part two: {}", count_valid);
    return count_valid;
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
