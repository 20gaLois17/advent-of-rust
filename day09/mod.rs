pub fn part_one(input: &str, preamble: usize) -> usize {
    let numbers: Vec<usize> = input.trim().split('\n').map(|e| e.parse().unwrap()).collect();
    for index in 0..numbers.len() {
        if index + 1 > preamble {
            if !has_sum_of_two(numbers[index], &numbers[index-preamble..index]) {
                println!("day09 -> part one: {}", numbers[index]);
                return numbers[index];
            }
        }
    }
    return 0;
}
fn has_sum_of_two(needle: usize, slice: &[usize]) -> bool {
    for i in 0..slice.len() {
        for k in (i+1)..slice.len() {
            if slice[i] + slice[k] == needle {
                return true
            }
        }
    }
    return false;
}
pub fn part_two(input: &str, preamble: usize) -> usize {
    let invalid_num: usize = part_one(input, preamble);
    let mut numbers: Vec<usize> = input.trim().split('\n').map(|e| e.parse().unwrap()).collect();
    for i in 0..numbers.len() {
        let mut sum: usize = 0;
        for k in i..numbers.len() {
            sum += numbers[k]; 
            if sum == invalid_num {
                let result: usize = sum_of_min_and_max(&mut numbers[i..k]);
                println!("day09 -> part two: {}", result); 
                return result;
            } else if sum > invalid_num { 
                break; 
            }
        }
    }
    return 0;
}
fn sum_of_min_and_max(arr: &mut[usize]) -> usize {
    arr.sort();
    return arr[0] + arr[arr.len() - 1];
}

#[cfg(test)]
mod tests {
    #[test]
    fn part_one() {
        assert_eq!(super::part_one(include_str!("testinput"), 5), 127);
    }
    #[test]
    fn part_two() {
        assert_eq!(super::part_two(include_str!("testinput"), 5), 62);
    }
}
