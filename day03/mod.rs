/**
 * We only need to store the positions of the trees (as a HashSet)
 * When we traverse the map we only need to check if we encounter a tree (i.e. we have a 'hit' in
 * the Hash Set ...)
 *
 * Since the map repeats itself to right side, we can compute the horizontal index of the position
 * by calculating the modulo between the interpolated horizontal distance and the width of the map
 */
use std::collections::HashSet;
pub fn part_one(input: &str) -> i64 {

    #[derive(Hash, Eq, PartialEq, Debug)]
    struct Tree {
        x: usize,
        y: usize,
    }
    struct Slope {
        h: usize,
        v: usize,
    }

    let mut trees: HashSet<Tree> = HashSet::new();

    let mut maxy :usize = 0;
    let mut maxx :usize = 0;
    for (y, line) in input.trim().split('\n').enumerate() {
        maxy += 1;
        for (x, c) in line.chars().enumerate() {
            if y == 0 {
                maxx += 1;
            }
            if c == '#' {
                trees.insert(Tree {x, y});
            }
        }
    }

    let slope:Slope = Slope { h: 3, v: 1 };

    let mut tree_encounters = 0;
    // if we treat current_position as a Tree struct, we have
    // an easy lookup in the hashmap ...
    let mut current_position:Tree = Tree { x: 0, y: 0 };

    // traverse the map along the given slope
    loop {
        current_position.x = (current_position.x + slope.h) % maxx;
        current_position.y += slope.v;
        // we have encountered a tree along the way...
        if trees.contains(&current_position) {
            tree_encounters += 1;
        }
        // we have traversed the given map
        if current_position.y >= maxy { break; }
    }
    println!("day03 -> part one: {}", tree_encounters);
    return tree_encounters;
}

/**
 * Basically the same as part one, but this time
 * we need to traverse the given map with multiples slopes
 * and multiply the tree_encounters after each pass
 */
pub fn part_two(input: &str) -> i64 {

    // slopes:
    // { h: 1, v: 1 }
    // { h: 3, v: 1 }
    // { h: 5, v: 1 }
    // { h: 7, v: 1 }
    // { h: 1, v: 2 }

    return 0;
}

#[cfg(test)]
mod tests {
    #[test]
    fn part_one() {
        assert_eq!(super::part_one(include_str!("testinput")), 7);
    }
    #[test]
    fn part_two() {
        assert_eq!(super::part_two(include_str!("testinput")), 336);
    }
}
