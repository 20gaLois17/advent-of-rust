use std::cmp;

pub fn part_one(input: &str) -> i64 {
    let mut result: i64 = 0;
    for line in input.trim().split('\n') {
        result = cmp::max(convert_to_seat_id(line), result);
    }
    println!("day05 -> part one: {}", result);
    return result;
}

/**
 * we have 839 / 1024 seats on the list and are looking for a missing_id
 * 1) the missing_id is not on the list
 * 2) missing_id + 1 and missing_id - 1 need to be on the list
 * 3) if that doesn't yield a unique solution, we also know that the front row and
 *    back row are empty (should automatically be inferred from 2) though)
 */
pub fn part_two(input: &str) -> i64 {
    //let mut result: i64 = 0;
    let seats: Vec<i64> = input.trim().split('\n').map(|e| convert_to_seat_id(e)).collect();
    let mut seat_candidates: Vec<i64> = vec![];
        for i in 0..1024 {
        if !seats.contains(&i) {
            if seats.contains(&(i-1)) && seats.contains(&(i+1)) {
                seat_candidates.push(i);
            }
        }
    }
    assert_eq!(seat_candidates.len(), 1);
    println!("day05 -> part two: {}", seat_candidates[0]);
    return 0;
}

pub fn convert_to_seat_id(val: &str) -> i64 {
    return bin_search([0, 127], ['F', 'B'], &val[..7]) * 8
        + bin_search([0, 7], ['L', 'R'], &val[7..]);
}

pub fn bin_search(arr: [i64; 2], bounds: [char; 2], val: &str) -> i64{
    let mut iter = val.chars();
    let c = iter.next().unwrap();

    // sanity checks
    assert!(arr[0] < arr[1]);
    assert!(bounds.contains(&c));

    if arr[0] + 1 == arr[1] {
        // exit condition
        return if c == bounds[0] {arr[0]} else {arr[1]};
    } else {
        // recurse ...
        let slice = if c == bounds[0] {
            [arr[0], (arr[0]+arr[1])/2] // use left half
        } else {
            [(arr[0]+arr[1]+1)/2, arr[1]] // use right half
        };
        return bin_search(slice, bounds, &val[1..]);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn seat_id_test() {
        assert_eq!(convert_to_seat_id("FBFBBFFRLR"), 357);
        assert_eq!(convert_to_seat_id("BFFFBBFRRR"), 567);
        assert_eq!(convert_to_seat_id("FFFBBBFRRR"), 119);
        assert_eq!(convert_to_seat_id("BBFFBBFRLL"), 820);
    }
    #[test]
    fn bin_search_test() {
        assert_eq!(bin_search([0, 1], ['F', 'B'], "F"), 0);
        assert_eq!(bin_search([0, 1], ['F', 'B'], "B"), 1);
        assert_eq!(bin_search([0, 7], ['F', 'B'], "FFF"), 0);
        assert_eq!(bin_search([0, 7], ['F', 'B'], "BBB"), 7);
        assert_eq!(bin_search([0, 7], ['F', 'B'], "BFB"), 5);
        assert_eq!(bin_search([0, 7], ['L', 'R'], "RLR"), 5);
    }
}
