pub fn part_one(input: &str) -> i64 {
    return 0;
}
pub fn part_two(input: &str) -> i64 {
    return 0;
}

// TODO: implement
pub fn convert_to_seat_id(val: &str) -> i64 {
    return 0;
}

#[cfg(test)]
mod tests {
    #[test]
    fn part_one() {
        assert_eq!(super::convert_to_seat_id("FBFBBFFRLR"), 357);
        assert_eq!(super::convert_to_seat_id("BFFFBBFRRR"), 567);
        assert_eq!(super::convert_to_seat_id("FFFBBBFRRR"), 119);
        assert_eq!(super::convert_to_seat_id("BBFFBBFRLL"), 820);

    }
    #[test]
    fn part_two() {
        assert_eq!(super::part_two(""), 1);
    }
}
