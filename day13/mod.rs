pub fn part_one(input: &str) -> usize {
    let lines: Vec<&str> = input.trim().split('\n').collect();
    let dep_time: i32 = lines[0].parse().unwrap();
    let shuttle_ids: Vec<i32> = lines[1].split(',')
        .filter(|e| e.to_string() != "x")
        .map(|e| e.parse().unwrap())
        .collect();

    let mut opt_id: i32 = i32::MAX;
    let mut opt_wait: i32 = i32::MAX;
    for id in shuttle_ids {
        let curr_wait: i32 = id - (dep_time % id);
        if curr_wait < opt_wait {
            opt_id = id;
            opt_wait = curr_wait;
        }
    }

    let result = (opt_id*opt_wait).try_into().unwrap();
    println!("day13 -> part one: {}", result);
    return result;
}
pub fn part_two(input: &str) -> usize {
    return 0;
}
#[cfg(test)]
mod tests {
    #[test]
    fn part_one() {
        assert_eq!(super::part_one(include_str!("testinput")), 295);
    }
    #[test]
    fn part_two() {
        assert_eq!(super::part_two(include_str!("testinput")), 286);
    }
}
