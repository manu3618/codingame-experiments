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

        let (row, col) = get_max_effect(&map);
        println!("{} {}", col, row);
    }
}

fn print_map(map: &Vec<Vec<char>>) {
    for line in map.iter().enumerate() {
        let (num, l) = line;
        eprintln!("{}\t{:?}", num, l.iter().collect::<String>());
    }
}

/// Return effect of explosion of bomb at (row, col)
/// Mutate map inplace.
fn blast_effect(map: &Vec<Vec<char>>, row: usize, col: usize) -> u32 {
    match map[row][col] {
        '#' | '@' => return 0,

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
                    // update map
                }
                '#' => break 'range,
                '3' | '2' | '1' | '0' => eprintln!("DEBUG 0780 : chain reaction"),
                '.' => (),
                _ => (),
            }
        }
    }
    return effect;
}

/// Return coordinates where blast effect is maximal
fn get_max_effect(map: &Vec<Vec<char>>) -> (usize, usize) {
    let mut max_effect = 0_u32;
    let mut which_max = (0_usize, 0_usize);
    let mut effect: u32;
    let height = map.len();
    let width = map[0].len();

    for (row, col) in iproduct!(0..height, 0..width) {
        effect = blast_effect(&map, row, col);
        if effect > max_effect {
            which_max = (row, col).clone();
            max_effect = effect;
        }
    }
    return which_max;
}
