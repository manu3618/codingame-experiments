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

    // game loop
    loop {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let inputs = input_line.split(" ").collect::<Vec<_>>();
        let rounds = parse_input!(inputs[0], i32); // number of rounds left before the end of the game
        let bombs = parse_input!(inputs[1], i32); // number of bombs left

        // Write an action using println!("message...");
        // To debug: eprintln!("Debug message...");

        let (row, col) = get_max_effect(&map, bombs);
        update_map(&mut map, row, col);
        println!("{} {}", col, row);
        eprintln!("DEBUG place bomb {} {}", col, row);
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
fn blast_effect(map: &Vec<Vec<char>>, row: usize, col: usize, left_bomb: i32) -> u32 {
    if left_bomb < 1 {
        return 0;
    }
    match map[row][col] {
        '#' | '@' | 'a'..='d' => return 0,
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
                '@' => {
                    effect = effect + 1;
                    // map[cell_row as usize][cell_col as usize] = '3'; // TODO update smartly
                }
                '#' => break 'range,
                '5'..='9' => {
                    effect = effect
                        + blast_effect(map, cell_row as usize, cell_col as usize, left_bomb - 1)
                }
                '.' => (),
                _ => (),
            }
        }
    }
    return effect;
}

/// Return coordinates where blast effect is maximal
fn get_max_effect(map: &Vec<Vec<char>>, left_bomb: i32) -> (usize, usize) {
    let mut max_effect = 0_u32;
    let mut which_max = (0_usize, 0_usize);
    let mut effect: u32;
    let height = map.len();
    let width = map[0].len();

    for (row, col) in iproduct!(0..height, 0..width) {
        effect = blast_effect(&map, row, col, left_bomb);
        if effect > max_effect {
            which_max = (row, col).clone();
            max_effect = effect;
        }
    }
    return which_max;
}

/// Update map in place
///
/// - put bomb at (row, col) (count down to explosion + 5)
/// - update countdown before explosion
/// - update cells
fn update_map(map: &mut Vec<Vec<char>>, row: usize, col: usize) {
    update_bomb(map, row, col);
    update_count(map);
}

fn update_bomb(map: &mut Vec<Vec<char>>, row: usize, col: usize) {
    let mut cell_row: i32;
    let mut cell_col: i32;
    let height = map.len() as i32;
    let width = map[0].len() as i32;

    match map[row][col] {
        '#' => eprintln!("DEBUG 1260 bomb should not be put at ({}, {})", row, col),
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
                '6' | '7' | '8' | '9' => update_bomb(map, cell_row as usize, cell_col as usize),
                '.' => map[cell_row as usize][cell_col as usize] = '4',
                '@' => map[cell_row as usize][cell_col as usize] = 'd',
                _ => (),
            }
        }
    }
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
