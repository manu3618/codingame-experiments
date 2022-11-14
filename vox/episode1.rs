// https://www.codingame.com/ide/puzzle/vox-codei-episode-1
use itertools::iproduct;
use std::io;

macro_rules! parse_input {
    ($x:expr, $t:ident) => {
        $x.trim().parse::<$t>().unwrap()
    };
}

/**
 * Auto-generated code below aims at helping you parse
 * the standard input according to the problem statement.
 **/
fn main() {
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let inputs = input_line.split(" ").collect::<Vec<_>>();
    let width = parse_input!(inputs[0], i32); // width of the firewall grid
    let height = parse_input!(inputs[1], i32); // height of the firewall grid
    let mut map: Vec<Vec<char>> = Vec::new();
    for i in 0..height as usize {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        // let map_row = input_line.trim_matches('\n').to_string(); // one line of the firewall grid
        input_line.pop(); // remove '\n'
        map.push(input_line.chars().collect());
    }
    print_map(&map);
    let mut first_move = true;

    // game loop
    loop {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let inputs = input_line.split(" ").collect::<Vec<_>>();
        let rounds = parse_input!(inputs[0], i32); // number of rounds left before the end of the game
        let bombs = parse_input!(inputs[1], i32); // number of bombs left

        // Write an action using println!("message...");
        // To debug: eprintln!("Debug message...");

        if first_move {
            // evaluate all move
            let mut min_turn = 100;
            let mut cur_turn = 0;
            let mut min_row = 0_usize;
            let mut min_col = 0_usize;
            for (row, col) in iproduct!(0..height as usize, 0..width as usize) {
                cur_turn = evaluate_first_move(&map, row, col, bombs, false);
                if cur_turn < min_turn {
                    (min_turn, min_row, min_col) = (cur_turn, row, col);
                }
            }
            eprintln!("DEBUG 0530 {} {}: {}", min_col, min_row, min_turn);
            first_move = false;
        }
        let (row, col, count) = get_max_effect_all(&map, bombs, false);
        let wait_count = evaluate_wait(&map, bombs);
        eprintln!("DEBUG 0440 wait {} {}", wait_count, count);
        if wait_count > count {
            update_count(&mut map);
            println!("WAIT");
        } else {
            update_map(&mut map, row, col);
            println!("{} {}", col, row);
            eprintln!("DEBUG place bomb {} {}", col, row);
        }
        print_map(&map);
    }
}

/// Print map
/// - . : nothing
/// - # :indestructible
/// - @ : target
/// - 1..4: empty cell that will be blasted soon
/// - 5..9: bomb that will explode soon
/// - a..d: target that will explode soon
fn print_map(map: &Vec<Vec<char>>) {
    for line in map.iter().enumerate() {
        let (num, l) = line;
        eprintln!("{}\t{:?}", num, l.iter().collect::<String>());
    }
}

/// Return effect of explosion of bomb at (row, col)
/// Mutate map inplace.
///
/// Params:
/// - map: map
/// - row, col: coordinate of the bomb
/// - wait: what if we wait for all bomb to explode before puting this one
fn blast_effect(map: &Vec<Vec<char>>, row: usize, col: usize, left_bomb: i32, wait: bool) -> u32 {
    if left_bomb < 1 {
        return 0;
    }
    match map[row][col] {
        '#' | '@' => return 0,
        'a'..='d' => {
            if !wait {
                return 0;
            }
        }
        _ => (),
    }
    let mut effect = 0_u32;
    let height = map.len() as i32 - 1;
    let width = map[0].len() as i32 - 1;
    let mut cell_row: i32;
    let mut cell_col: i32;
    let directions = [(0_i32, 1_i32), (0, -1), (1, 0), (-1, 0)];

    'dir: for direction in directions {
        'range: for range in (1..4) {
            cell_row = row as i32 + direction.0 * range;
            if cell_row < 0 || cell_row > height {
                continue;
            }
            cell_col = col as i32 + direction.1 * range;
            if cell_col < 0 || cell_col > width {
                continue;
            }

            match map[cell_row as usize][cell_col as usize] {
                '@' => effect = effect + 1,
                '#' => break 'range,
                '5'..='9' => {
                    if !wait {
                        effect = effect
                            + blast_effect(
                                map,
                                cell_row as usize,
                                cell_col as usize,
                                left_bomb - 1,
                                wait,
                            )
                    }
                }
                'a'..='d' => (),
                '.' => (),
                _ => (),
            }
        }
    }
    return effect;
}

/// Return coordinates where blast effect is maximal
fn get_which_max_effect(map: &Vec<Vec<char>>, left_bomb: i32, wait: bool) -> (usize, usize) {
    let (row, col, count) = get_max_effect_all(map, left_bomb, false);
    return (row, col);
}

fn get_max_effect(map: &Vec<Vec<char>>, left_bomb: i32, wait: bool) -> u32 {
    let (row, col, count) = get_max_effect_all(map, left_bomb, wait);
    return count;
}

fn get_max_effect_all(map: &Vec<Vec<char>>, left_bomb: i32, wait: bool) -> (usize, usize, u32) {
    let mut max_effect = 0_u32;
    let mut which_max = (0_usize, 0_usize);
    let mut effect: u32;
    let height = map.len();
    let width = map[0].len();

    for (row, col) in iproduct!(0..height, 0..width) {
        effect = blast_effect(&map, row, col, left_bomb, wait);
        if effect > max_effect {
            which_max = (row, col).clone();
            max_effect = effect;
        }
    }
    return (which_max.0, which_max.1, max_effect);
}

/// Update map in place
///
/// - put bomb at (row, col) (count down to explosion + 5)
/// - update countdown before explosion
/// - update cells
fn update_map(map: &mut Vec<Vec<char>>, row: usize, col: usize) -> Result<i32, i32> {
    match update_bomb(map, row, col) {
        Ok(_) => (),
        Err(x) => return Err(x),
    }
    update_count(map);
    return Ok(0);
}

fn update_bomb(map: &mut Vec<Vec<char>>, row: usize, col: usize) -> Result<i32, i32> {
    let mut cell_row: i32;
    let mut cell_col: i32;
    let height = map.len() as i32;
    let width = map[0].len() as i32;

    match map[row][col] {
        '#' => {
            eprintln!("DEBUG 1260 bomb should not be put at ({}, {})", row, col);
            return Err(0);
        }
        '.' => map[row][col] = '9',
        _ => (),
    }

    let directions = [(0_i32, 1_i32), (0, -1), (1, 0), (-1, 0)];
    'dir: for direction in directions {
        'range: for range in (1..4) {
            cell_row = row as i32 + direction.0 * range;
            if cell_row < 0 || cell_row >= height {
                continue;
            }
            cell_col = col as i32 + direction.1 * range;
            if cell_col < 0 || cell_col >= width {
                continue;
            }

            match map[cell_row as usize][cell_col as usize] {
                '#' => break 'range,
                '6' | '7' | '8' | '9' => {
                    update_bomb(map, cell_row as usize, cell_col as usize);
                }
                '.' => map[cell_row as usize][cell_col as usize] = '4',
                '@' => map[cell_row as usize][cell_col as usize] = 'd',
                _ => (),
            }
        }
    }
    return Ok(0);
}

fn update_count(map: &mut Vec<Vec<char>>) {
    let height = map.len();
    let width = map[0].len();
    let mut count: i32;

    for (row, col) in iproduct!(0..height, 0..width) {
        match map[row][col] {
            '1'..='4' | '6'..='9' => {
                count = map[row][col].to_string().parse().unwrap();
                map[row][col] = format!("{}", count - 1).chars().next().unwrap();
            }
            '5' | '0' | 'a' => map[row][col] = '.',
            'b' => map[row][col] = 'a',
            'c' => map[row][col] = 'b',
            'd' => map[row][col] = 'c',
            _ => (),
        }
    }
}

fn evaluate_wait(map: &Vec<Vec<char>>, left_bomb: i32) -> u32 {
    return get_max_effect(map, left_bomb, true);
}

/// What if fisrt move is this one?
/// Return number of turn needed
fn evaluate_first_move(
    init_map: &Vec<Vec<char>>,
    init_row: usize,
    init_col: usize,
    bombs: i32,
    print: bool,
) -> i32 {
    let mut map = init_map.clone();
    let height = map.len();
    let width = map[0].len();
    let mut bomb_left = true;
    match update_map(&mut map, init_row, init_col) {
        Ok(_) => (),
        Err(_) => return 1000,
    }
    for turn in 1..(3 * bombs + 5) as usize {
        let (row, col, count) = get_max_effect_all(&map, bombs, false);
        let wait_count = evaluate_wait(&map, bombs);
        if wait_count > count {
            if print {
                println!("WAIT");
            }
            update_count(&mut map);
        } else {
            if print {
                println!("{} {}", row, col);
            }
            update_map(&mut map, row, col);
        }

        bomb_left = false;
        for (row, col) in iproduct!(0..height, 0..width) {
            if map[row][col] == '@' {
                bomb_left = true;
                break;
            }
        }
        if !bomb_left {
            return turn as i32;
        }
    }
    return 1000;
}
