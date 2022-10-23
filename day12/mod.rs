pub fn part_one(input: &str) -> usize {
    let mut pos: [i32; 2] = [0, 0];
    let mut deg: i32 = 90;

    for line in input.trim().split('\n') {
        let num: i32 = line[1..].parse().unwrap();
        let instr: char = line.chars().next().unwrap();
        match instr {
            'N' => pos[0] += num,
            'S' => pos[0] -= num, 
            'E' => pos[1] += num,
            'W' => pos[1] -= num, 
            'L' => deg = (deg - num + 360) % 360, 
            'R' => deg = (deg + num + 360) % 360, 
            'F' => match deg {
                0   => pos[0] += num,
                90  => pos[1] += num,
                180 => pos[0] -= num,
                270 => pos[1] -= num,
                _   => panic!("unknown direction")
            }, 
            _   => panic!("unknown instruction")
        }
    }

    let result: usize = (pos[0].abs() + pos[1].abs()).try_into().unwrap();
    println!("day12 -> part one: {}", result);
    return result;
}
pub fn part_two(input: &str) -> usize {
    for line in input.trim().split('\n') {
    }
    return 0;
}

#[cfg(test)]
mod tests {
    #[test]
    fn part_one() {
        assert_eq!(super::part_one(include_str!("testinput")), 25);
    }
    #[test]
    fn part_two() {
        assert_eq!(super::part_two(include_str!("testinput")), 1);
    }
}
