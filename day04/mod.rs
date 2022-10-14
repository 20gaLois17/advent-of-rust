use regex::Regex;

pub fn part_one(input: &str) -> i64 {
    let mut result: i64 = 0;
    for line in input.trim().split("\n\n") {
        let fields: Vec<&str> = line.split_whitespace().collect();
        let mut inc: bool = true;
        if fields.len() < 7 {
            inc = false;
        }
        if fields.len() == 7 {
            for field in fields {
                if field.contains("cid:") {
                    inc = false;
                }
            }
        }
        result += if inc {1} else {0}
    }
    println!("day04 -> part one: {}", result);
    return result;
}

pub fn part_two(input: &str) -> i64 {
    let mut result: i64 = 0;
    for line in input.trim().split("\n\n") {
        let mut inc: bool = true;
        let fields:Vec<&str> = line.split_whitespace().collect();
        let fields_count = fields.len();
        if fields_count < 7 {
            inc = false;
        }
        for field in fields {
            inc = inc & is_valid_field(field, fields_count);
        }
        result += if inc {1} else {0};
    }

    println!("day04 -> part two: {}", result);
    return result;
}

enum Field {
    Birthyear,
    Issueyear,
    ExpirationYear,
    Height,
    HairColor,
    EyeColor,
    PassportId,
    CountryId
}

impl Field {
    fn as_str(&self) -> &'static str {
        match self {
            Field::Birthyear      => "byr",
            Field::Issueyear      => "iyr",
            Field::ExpirationYear => "eyr",
            Field::Height         => "hgt",
            Field::HairColor      => "hcl",
            Field::EyeColor       => "ecl",
            Field::PassportId     => "pid",
            Field::CountryId      => "cid",
        }
    }

    fn is_valid(&self, val: &str) -> bool {
        match self {
            Field::CountryId      => true, // this field does not need to be checked ever
            Field::Birthyear      => in_range_of(val, 1920, 2002),
            Field::Issueyear      => in_range_of(val, 2010, 2020),
            Field::ExpirationYear => in_range_of(val, 2020, 2030),
            Field::EyeColor       => ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].iter().any(|e| e == &val),
            Field::PassportId     => val.chars().count() == 9 && val.chars().all(|c| c.is_digit(10)),
            Field::HairColor      => Regex::new(r"^#[a-f0-9]{6}$").unwrap().is_match(val),
            Field::Height         => {
                let unit = &val[val.len()-2..val.len()];
                if unit == "cm" {
                    return in_range_of(&val[..val.len()-2], 150, 193);
                } else if unit == "in" {
                    return in_range_of(&val[..val.len()-2], 59, 76);
                }
                return false;
            },
        }
    }
}

// helper functions
fn is_valid_field(field: &str, field_count: usize) -> bool {
    let sides: Vec<&str> = field.split(":").collect();
    let field_name  = sides[0];
    let field_value = sides[1];

    // we could probably use 'match' for this but I don't know how yet
    if field_name == Field::Birthyear.as_str() {
        return Field::Birthyear.is_valid(field_value);
    }
    else if field_name == Field::Issueyear.as_str() {
        return Field::Issueyear.is_valid(field_value);
    }
    else if field_name == Field::ExpirationYear.as_str() {
        return Field::ExpirationYear.is_valid(field_value);
    }
    else if field_name == Field::Height.as_str() {
        return Field::Height.is_valid(field_value);
    }
    else if field_name == Field::HairColor.as_str() {
        return Field::HairColor.is_valid(field_value);
    }
    else if field_name == Field::EyeColor.as_str() {
        return Field::EyeColor.is_valid(field_value);
    }
    else if field_name == Field::PassportId.as_str() {
        return Field::PassportId.is_valid(field_value);
    }
    else if field_name == Field::CountryId.as_str() {
        return field_count == 8;
    }
    panic!("{} did not match any field", field);
}

fn digits_only(val: &str) -> bool {
    for c in val.chars() {
        if !c.is_ascii_digit() {
            return false;
        }
    }
    return true;
}
fn in_range_of(val: &str, lb: i32, ub: i32) -> bool {
    if !digits_only(val) {
        return false;
    }
    match val.parse::<i32>() {
        Err(_)  => return false,
        Ok(num) => return num >= lb && num <= ub,
    }
}


#[cfg(test)]
mod tests {
    #[test]
    fn part_one() {
        assert_eq!(super::part_one(include_str!("testinput")), 2);
    }
    #[test]
    fn part_two() {
        assert_eq!(super::part_two(include_str!("testinput_invalid")), 0);
        assert_eq!(super::part_two(include_str!("testinput_valid")), 4);
    }

    // unit tests
    #[test]
    fn digits_only() {
        assert_eq!(super::digits_only("01231532"), true);
        assert_eq!(super::digits_only("a1532"), false);
        assert_eq!(super::digits_only("0000001"), true);
        assert_eq!(super::digits_only(""), true);
        assert_eq!(super::digits_only("-ab10"), false);
    }
    #[test]
    fn in_range_of() {
        assert_eq!(super::in_range_of("30", 30, 42), true);
        assert_eq!(super::in_range_of("42", 30, 42), true);
        assert_eq!(super::in_range_of("33", 30, 42), true);
        assert_eq!(super::in_range_of("28", 30, 42), false);
        assert_eq!(super::in_range_of("45", 30, 42), false);
    }
    #[test]
    fn test_birthyear() {
        assert!(super::Field::Birthyear.is_valid("1999"));
        assert!(super::Field::Birthyear.is_valid("2000"));
        assert!(!super::Field::Birthyear.is_valid("2003"));
        assert!(!super::Field::Birthyear.is_valid(""));
    }
    #[test]
    fn test_eyecolor() {
        assert!(super::Field::EyeColor.is_valid("amb"));
        assert!(super::Field::EyeColor.is_valid("blu"));
        assert!(!super::Field::EyeColor.is_valid("test"));
        assert!(!super::Field::EyeColor.is_valid("grey"));
    }
    #[test]
    fn test_height() {
        assert!(super::Field::Height.is_valid("60in"));
        assert!(super::Field::Height.is_valid("190cm"));
        assert!(!super::Field::Height.is_valid("190in"));
        assert!(!super::Field::Height.is_valid("190"));
        assert!(!super::Field::Height.is_valid("60"));
    }
    #[test]
    fn test_passport_id() {
        assert!(super::Field::PassportId.is_valid("000000001"));
        assert!(super::Field::PassportId.is_valid("001302091"));
        assert!(super::Field::PassportId.is_valid("000000121"));
        assert!(!super::Field::PassportId.is_valid("0000000A1"));
        assert!(!super::Field::PassportId.is_valid("0000000001"));
    }
    #[test]
    fn test_haircolor() {
        assert!(super::Field::HairColor.is_valid("#123abc"));
        assert!(super::Field::HairColor.is_valid("#efefef"));
        assert!(!super::Field::HairColor.is_valid("#123abz"));
        assert!(!super::Field::HairColor.is_valid("123abc"));
    }

}
