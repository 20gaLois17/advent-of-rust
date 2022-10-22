use std::collections::HashSet;

pub fn part_one(input: &str) -> usize {
    let numbers = create_sorted_adapter_array(input);
    let mut delta_1 = 0;
    let mut delta_3 = 0;
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
    let numbers = create_sorted_adapter_array(input);

    let mut delta_1 = 0;
    let mut delta_3 = 0;
    let mut result  = 1; // we have at least one sulution
                         // all potential solutions are multiplicative
    for i in 0..numbers.len()-1 {
        match numbers[i+1] - numbers[i] {
            1 => delta_1 += 1,
            3 => {
                result *= subchain_count(delta_1);
                delta_1 = 0;
            }
            _ => panic!("not a valid delta")
        }
    }

    println!("day10 -> part two: {}", result);
    return result;
}

/** 
 * 1,4,7 (delta_1 = 0) is not breakable
 * 1,4,5,8 (delta_1 = 1) is not breakable
 * 1,4,5,6,9 (delta_1 = 2) can be split in two (5 can be omitted)
 * 1,4,5,6,7,10 can be split in 4 (5 or 6 can be omitted)
 * 1,4,5,6,7,8,11 can be split in 7 viable chains
 * ... we could not detect longer (breakable) chains than that in input
 */
fn subchain_count(val: usize) -> usize {
    match val {
        0 => 1,
        1 => 1,
        2 => 2,
        3 => 4,
        4 => 7,
        _ => panic!("foo cannot parse this number {}", val)
    }
}

fn create_sorted_adapter_array(input: &str) -> Vec<usize> {
    let mut numbers: Vec<usize> = input.trim().split('\n')
        .map(|e| e.parse().unwrap())
        .collect();
    numbers.push(0); // we need to account for implicit lowest adapter
    numbers.sort();
    numbers.push(numbers[numbers.len()-1] + 3); // we need to account for highest adapter
    return numbers;
}

#[cfg(test)]
mod tests {
    #[test]
    fn part_one() {
        assert_eq!(super::part_one(include_str!("testinput")), 220);
    }
    #[test]
    fn part_two() {
        assert_eq!(super::part_two(include_str!("testinput")), 19208);
    }
}
