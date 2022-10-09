pub fn part_one(input: &str) -> i64 {

    const TARGET: i64 = 2020;
    let mut values: [i64; 200] = [10000; 200];
    let mut i = 0;
    for line in input.trim().split('\n') {
        values[i] = line.parse::<i64>().unwrap();
        i += 1; 
    }
    for i in 0..values.len() {
        for y in i..values.len() {
            if values[i] + values[y] == TARGET {
                println!("day01 -> part one: {}", values[i] * values[y]);
                return values[i] * values[y]; 
            }
        }
    }
    return -1; // did not find a match
}
pub fn part_two(input: &str) -> i64 {
    const TARGET: i64 = 2020;
    let mut values: [i64; 200] = [10000; 200];
    let mut i = 0;
    for line in input.trim().split('\n') {
        values[i] = line.parse::<i64>().unwrap();
        i += 1; 
    }

    for i in 0..values.len() {
        for y in i..values.len() {
            for k in y..values.len() {
                if values[i] + values[y] + values[k] == TARGET {
                    println!("day01 -> part two: {}", values[i] * values[y] * values[k]);
                    return values[i] * values[y] * values[k];
                }
            }
        }
    }
    return -1; // did not find a match
}

#[cfg(test)]
mod tests {
    #[test]
    fn part_one() {
        assert_eq!(super::part_one(include_str!("testinput")), 514579);
    }
    #[test]
    fn part_two() {
        assert_eq!(super::part_two(include_str!("testinput")), 241861950);
    }
}
