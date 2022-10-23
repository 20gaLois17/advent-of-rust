pub fn part_one(input: &str) -> usize {
    let mut board: Vec<Vec<char>> = input.trim().split('\n')
        .map(|s| s.chars().collect())
        .collect();

    // setup and first run
    let mut updates: Vec<[usize; 2]> = vec![];
    for (i, row) in board.iter().enumerate() {
        for (k, _) in row.iter().enumerate() {
            if will_change(&board, i, k) {
                updates.push([i, k]);
            }
        }
    }

    while updates.len() > 0 {
        for update in updates.iter() {
            let row_index: usize = update[0];
            let col_index: usize = update[1];
            match board[row_index][col_index] {
                'L' => board[row_index][col_index] = '#',
                '#' => board[row_index][col_index] = 'L',
                _   => panic!("unidentifiable board char")

            }
        }

        //for line in &board {
        //    for c in line {
        //        print!("{}", c);
        //    }
        //    print!("\n");
        //}
        //println!();

        updates = vec![];
        for (i, row) in board.iter().enumerate() {
            for (k, _) in row.iter().enumerate() {
                if will_change(&board, i, k) {
                    updates.push([i, k]);
                }
            }
        }
    }
    let result: usize = count_occupied_seats(&board);

    println!("day11 -> part one: {}", result);
    return result;
}


fn count_occupied_seats(board: &Vec<Vec<char>>) -> usize {
    let mut result: usize = 0;
    for line in board {
        for c in line {
            match c {
                '#' => result += 1,
                'L' => {},
                '.' => {},
                _   => panic!("unidentifiable board char")
            }
        }
    }
    return result;
}

fn will_change(board: &Vec<Vec<char>>, i: usize, k: usize) -> bool {
    match board[i][k] {
        '.' => false,
        'L' => count_surrounding_occupied_seats(&board, i, k) == 0,
        '#' => count_surrounding_occupied_seats(&board, i, k) > 3,
        _   => panic!("unknown board character")
    }
}

pub fn part_two(input: &str) -> usize {
    let mut board: Vec<Vec<char>> = input.trim().split('\n')
        .map(|s| s.chars().collect())
        .collect();

    // setup and first run
    let mut updates: Vec<[usize; 2]> = vec![];
    for (i, row) in board.iter().enumerate() {
        for (k, _) in row.iter().enumerate() {
            if will_change_two(&board, i, k) {
                updates.push([i, k]);
            }
        }
    }
    //for i in 0..1 {
    while updates.len() > 0 {
        for update in updates.iter() {
            let row_index: usize = update[0];
            let col_index: usize = update[1];
            match board[row_index][col_index] {
                'L' => board[row_index][col_index] = '#',
                '#' => board[row_index][col_index] = 'L',
                _   => panic!("unidentifiable board char")

            }
        }

        //for line in &board {
        //    for c in line {
        //        print!("{}", c);
        //    }
        //    print!("\n");
        //}
        //println!();

        updates = vec![];
        for (i, row) in board.iter().enumerate() {
            for (k, _) in row.iter().enumerate() {
                if will_change_two(&board, i, k) {
                    updates.push([i, k]);
                }
            }
        }
    }
    let result: usize = count_occupied_seats(&board);

    println!("day11 -> part one: {}", result);
    return result;
}

fn will_change_two(board: &Vec<Vec<char>>, i: usize, k: usize) -> bool {
    match board[i][k] {
        '.' => false,
        'L' => count_visible_occupied_seats(&board, i, k) == 0,
        '#' => count_visible_occupied_seats(&board, i, k) > 4,
        _   => panic!("unknown board character")
    }
}

fn count_surrounding_occupied_seats(board: &Vec<Vec<char>>, i: usize, k: usize) -> usize {
    let board_rows: i32 = board.len().try_into().unwrap();
    let board_cols: i32 = board[0].len().try_into().unwrap();
    let row_index: i32 = i.try_into().unwrap();
    let col_index: i32 = k.try_into().unwrap();

    let offsets: Vec<[i32; 2]> = vec![
        [-1, 0],   // top
        [0, -1],   // left
        [1, 0],    // bottom
        [0, 1],    // right
        [-1, -1],  // top left
        [-1, 1],   // top right
        [1, -1],   // bottom left
        [1, 1]     // bottom right
    ];
    let mut seat_count: usize = 0;
    for offset in offsets {
        let x = row_index + offset[0];
        let y = col_index + offset[1];
        if !(x >= 0 && x < board_rows) {
            continue;
        }
        if !(y >= 0 && y < board_cols) {
            continue;
        }
        match board[x as usize][y as usize] {
            '.' => {},
            'L' => {},
            '#' => seat_count += 1,
            _   => panic!("unidentifiable symbol")

        }
    }
    return seat_count;
}

fn count_visible_occupied_seats(board: &Vec<Vec<char>>, i: usize, k: usize) -> usize {
    let board_rows: i32 = board.len().try_into().unwrap();
    let board_cols: i32 = board[0].len().try_into().unwrap();
    let row_index: i32 = i.try_into().unwrap();
    let col_index: i32 = k.try_into().unwrap();

    let offsets: Vec<[i32; 2]> = vec![
        [-1, 0],   // top
        [0, -1],   // left
        [1, 0],    // bottom
        [0, 1],    // right
        [-1, -1],  // top left
        [-1, 1],   // top right
        [1, -1],   // bottom left
        [1, 1]     // bottom right
    ];
    let mut seat_count: usize = 0;
    for offset in offsets {
        let x = row_index + offset[0];
        let y = col_index + offset[1];
        if !(x >= 0 && x < board_rows) {
            continue;
        }
        if !(y >= 0 && y < board_cols) {
            continue;
        }
        match board[x as usize][y as usize] {
            '.' => if found_occupied_in_direction(&board, i ,k, offset) {
                seat_count += 1;
            },
            'L' => {},
            '#' => seat_count += 1,
            _   => panic!("unidentifiable symbol")

        }
    }
    return seat_count;
}

pub fn found_occupied_in_direction(board: &Vec<Vec<char>>, i: usize, k: usize, offset: [i32; 2]) -> bool {
    let board_rows: i32 = board.len().try_into().unwrap();
    let board_cols: i32 = board[0].len().try_into().unwrap();
    let row_index: i32 = i.try_into().unwrap();
    let col_index: i32 = k.try_into().unwrap();

    let mut x = row_index + offset[0];
    let mut y = col_index + offset[1];

    while board[x as usize][y as usize] == '.' {
        x  += offset[0];
        y  += offset[1];

        if !(x >= 0 && x < board_rows) {
            return false;
        }
        if !(y >= 0 && y < board_cols) {
            return false;
        }
    }

    return board[x as usize][y as usize] == '#'

}



#[cfg(test)]
mod tests {
    #[test]
    fn part_one() {
        assert_eq!(super::part_one(include_str!("testinput")), 37);
    }
    #[test]
    fn part_two() {
        assert_eq!(super::part_two(include_str!("testinput")), 26);
    }
}
