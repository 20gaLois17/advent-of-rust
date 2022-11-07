use std::collections::HashMap;
use regex::Regex;

pub fn part_one(input: &str) -> usize {
    let lines: Vec<&str> = input.trim().split('\n').collect();
    // mask to be applied to the input value
    let mut curr_mask: String = "xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx".to_string();
    // regex to extract the numbers from 'mem[123]'
    let mem_regex: Regex = Regex::new(r"([0-9]+)").unwrap();
    // hashmap to store the memory
    let mut memory: HashMap<usize, usize> = HashMap::new();

    for line in lines {
        let chunks: Vec<&str> = line.split('=').collect();
        if chunks[0].contains("mask") {
            // new mask
            curr_mask = chunks[1].trim().to_string();
        } else {
            // memory operation
            let mem_addr: usize = mem_regex.find(chunks[0])
                .unwrap()
                .as_str()
                .parse()
                .unwrap();
            let value: usize = chunks[1].trim().parse().unwrap();
            // apply the mask to the bitstring
            let masked_value = apply_mask(&curr_mask, to_36_bitstring(value));
            //println!("bit value:    {}", to_36_bitstring(value));
            //println!("masked_value: {}", masked_value);
            memory.insert(mem_addr, from_bitstring(masked_value));
        }
    }

    let mut result: usize = 0;
    for (_key, val) in memory {
        result = result + val;
    }
    println!("day14 -> part one: {}", result);
    return result;
}
pub fn part_two(input: &str) -> usize {
    return 0;
}

fn to_36_bitstring(num: usize) -> String {
    return format!("{num:036b}");
}
fn from_bitstring(num: String) -> usize {
    return usize::from_str_radix(&num, 2).unwrap();
}

fn apply_mask(mask: &String, bits: String) -> String {
    assert_eq!(mask.len(), bits.len());

    let chars: Vec<char> = bits.chars().enumerate().map(|(i, c)| {
        let mc = mask.chars().nth(i).unwrap();
        if mc != 'X' {
            return mc
        } else {
            return c
        }
    }).collect();
    return chars.iter().collect();
}

fn apply_mask_part_two(mask: &String, bits: String) -> String {
    assert_eq!(mask.len(), bits.len());
    let chars: Vec<char> = bits.chars().enumerate().map(|(i, c)| {
        let mc = mask.chars().nth(i).unwrap();
        if mc == '0' {
            return c
        } else {
            return mc
        }
    }).collect();
    return chars.iter().collect();
}

fn expand_from_floating_bits(bits: String) -> Vec<String> {
    let mut bitstrings: Vec<String> = vec![];
    let indices: Vec<usize> = get_floating_point_indices(bits.clone());
    // now we need all permutations
    for i in indices {
        //bits.replace_range(i+1..i+2, "1"); 
    }
    bitstrings.push(bits);
    return bitstrings;

}
fn get_floating_point_indices(bits: String) -> Vec<usize> {
    let mut indices_list: Vec<usize> = vec![];
    for (i, c) in bits.chars().enumerate() {
        if c == 'X' {
            indices_list.push(i);
        }
    }
    // sanity check
    for i in &indices_list {
        assert!(bits.chars().nth(*i).unwrap() == 'X');
    }
    return indices_list;
}

fn contains_floating_bits(bits: String) -> bool {
    for c in bits.chars() {
        if c == 'X' {
            return true;
        }
    }
    return false;
}

#[cfg(test)]
mod tests {
    use regex::Regex;
    #[test]
    fn part_one() {
        assert_eq!(super::part_one(include_str!("testinput")), 165);
    }
    #[test]
    fn part_two() {
        assert_eq!(super::part_two(include_str!("testinput")), 208);
    }
    #[test]
    fn regex_test() {
        assert_eq!(Regex::new(r"([0-9]+)").unwrap().find("mem[21]").unwrap().as_str(), "21");
    }
    #[test]
    fn apply_mask() {
        assert_eq!(super::apply_mask(&"0101".to_string(), "1111".to_string()), "0101");
        assert_eq!(super::apply_mask(&"XXXX".to_string(), "1010".to_string()), "1010");
        assert_eq!(super::apply_mask(&"XXX1".to_string(), "0000".to_string()), "0001");
    }
    #[test]
    fn apply_mask_part_two() {
        assert_eq!(super::apply_mask_part_two(&"0101".to_string(), "1111".to_string()), "1111");
        assert_eq!(super::apply_mask_part_two(&"XXXX".to_string(), "1010".to_string()), "XXXX");
        assert_eq!(super::apply_mask_part_two(&"XXX1".to_string(), "0000".to_string()), "XXX1");
    }
    #[test]
    fn contains_floating_bits() {
        assert_eq!(super::contains_floating_bits("010100".to_string()), false);
        assert_eq!(super::contains_floating_bits("11111".to_string()), false);
        assert_eq!(super::contains_floating_bits("X10000".to_string()), true);
    }
    #[test]
    fn get_floating_point_indices() {
        assert_eq!(super::get_floating_point_indices("0000".to_string()).len(), 0);
        assert_eq!(super::get_floating_point_indices("X00X".to_string()).len(), 2);
        assert_eq!(super::get_floating_point_indices("X0X0".to_string()), vec![0,2]);

    }
    #[test]
    fn expand_from_floating_bits() {
    }
}

