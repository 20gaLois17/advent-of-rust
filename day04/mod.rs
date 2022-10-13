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
        if inc {
            result += 1;
        }
    }
    println!("day04 -> part one: {}", result);
    return result;
}

pub fn part_two(input: &str) -> i64 {
    let mut result: i64 = 0;
    let mut inc: bool = true;
    for line in input.trim().split("\n\n") {
        let fields:Vec<&str> = line.split_whitespace().collect();

        if fields.len() < 7 {
            continue;
        }
        for field in fields {
            inc = inc & is_valid(field); // only short circuit after testing is done
        }
        if inc {
            result += 1;
        }
    }
    return 0;
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
}

// helper functions
fn is_valid(field: &str) -> bool {
    let sides: Vec<&str> = field.split(":").collect();

    // we could probably use 'match' for this but I don't know how yet
    if sides[0] == Field::Birthyear.as_str() {
        if !digits_only(sides[1]) {
            return false;
        } else {
            return in_range_of(sides[1], 1920, 2002);
        }
    }
    else if sides[0] == Field::Issueyear.as_str() {
        if !digits_only(sides[1]) {
            return false;
        } else {
            return in_range_of(sides[1], 2010, 2020);
        }
    }
    // TODO: implement
    else if sides[0] == Field::ExpirationYear.as_str() {
    }
    // TODO: implement
    else if sides[0] == Field::Height.as_str() {
    }
    // TODO: implement
    else if sides[0] == Field::HairColor.as_str() {
    }
    // TODO: implement
    else if sides[0] == Field::EyeColor.as_str() {
    }
    // TODO: implement
    else if sides[0] == Field::PassportId.as_str() {
    }
    // TODO: implement
    else if sides[0] == Field::CountryId.as_str() {
    }
    return false;
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
    match val.parse::<i32>() {
        Err(_) => return false,
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
        assert_eq!(super::part_two(include_str!("testinput")), 1);
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
}
