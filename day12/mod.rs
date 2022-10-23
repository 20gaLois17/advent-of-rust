pub fn part_one(input: &str) -> usize {
    let mut pos: [i32; 2] = [0, 0]; // starting position of ship
    let mut deg: i32 = 90;          // starting orientation of ship

    for line in input.trim().split('\n') {
        let num: i32 = line[1..].parse().unwrap();
        let instr: char = line.chars().next().unwrap();
        match instr {
            'N' => pos[0] += num,
            'S' => pos[0] -= num, 
            'E' => pos[1] += num,
            'W' => pos[1] -= num, 
            'L' => deg = (deg - num + 360) % 360, // we need the (unique) positive remainder
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

    let result = get_manhattan_dist(pos);
    println!("day12 -> part one: {}", result);
    return result;
}

fn get_manhattan_dist(pos: [i32; 2]) -> usize {
    return (pos[0].abs() + pos[1].abs()).try_into().unwrap();
}

pub fn part_two(input: &str) -> usize {
    let mut pos: [i32; 2] = [0, 0];  // starting position of ship
    let mut way: [i32; 2] = [1, 10]; // starting position of waypoint
    for line in input.trim().split('\n') {
        let num: i32 = line[1..].parse().unwrap();
        let instr: char = line.chars().next().unwrap();
        match instr {
            'N' => way[0] += num,
            'S' => way[0] -= num, 
            'E' => way[1] += num,
            'W' => way[1] -= num, 
            'L' => match num { 
                0   => {},
                90  => {
                    way = [way[1], -way[0]];
                },
                180 => {
                    way = [-way[0], -way[1]];
                },
                270 => {
                    way = [-way[1], way[0]];
                },
                _   => panic!("unknown direction")
            }, // rotate waypoint counter clock wise
            'R' => match num {
                0   => {},
                90  => {
                    way = [-way[1], way[0]];
                },
                180 => {
                    way = [-way[0], -way[1]];
                },
                270 => {
                    way = [way[1], -way[0]];
                },
                _   => panic!("unknown direction")
            }, // rotate waypoint clock wise
            'F' => {
                pos[0] += num*way[0];
                pos[1] += num*way[1];
            }, // move ship in relation to waypoint
            _   => panic!("unknown instruction")
        }
    }

    let result = get_manhattan_dist(pos);
    println!("day12 -> part two: {}", result);
    return result;
}

#[cfg(test)]
mod tests {
    #[test]
    fn part_one() {
        assert_eq!(super::part_one(include_str!("testinput")), 25);
    }
    #[test]
    fn part_two() {
        assert_eq!(super::part_two(include_str!("testinput")), 286);
    }
}
