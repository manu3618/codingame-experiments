// https://www.codingame.com/ide/puzzle/brain-fork
use std::io;

static VALUES: [char; 27] = [
    ' ', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R',
    'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
];

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
    eprintln!("Debug Message...");
    io::stdin().read_line(&mut input_line).unwrap();
    let magic_phrase = input_line.trim_matches('\n').to_string();
    let letters: Vec<char> = magic_phrase.chars().collect(); // wanted output
    let mut rune = 'A';
    let mut forest = [' '; 30];
    let mut output_letter = "".to_string();
    let mut idx: usize = 0;
    let mut wanted_idx: usize;
    let mut init_value = forest[idx];

    // Write an action using println!("message...");
    // To debug: eprintln!("Debug message...");
    for position in 0..letters.len() {
        wanted_idx = get_minimal_rune_idx(letters[position], &forest, idx);
        output_letter += &get_move(wanted_idx, idx, forest.len() as i32);
        idx = wanted_idx;
        init_value = forest[idx];
        output_letter += &get_letter(letters[position], &init_value);
        forest[idx] = letters[position];
        output_letter += ".";
    }
    eprintln!("output length {}", output_letter.len());
    println!("{}", output_letter);
}

/// Get part of string needed to obtain the new letter
/// run is modified in place
fn get_letter(wanted_letter: char, rune: &char) -> String {
    let rune_pos = VALUES.iter().position(|&x| x == *rune).unwrap();
    let wanted_pos = VALUES.iter().position(|&x| x == wanted_letter).unwrap();
    let dist = get_letter_dist(wanted_letter, *rune);
    if dist < 0 {
        return "-".repeat(dist.abs() as usize);
    } else {
        return "+".repeat(dist as usize);
    }
}

/// Get string to move from one rune to another
fn get_move(wanted_idx: usize, current_idx: usize, forest_len: i32) -> String {
    let dist = get_mod_dist(current_idx as i32, wanted_idx as i32, forest_len);
    if dist < 0 {
        return "<".repeat(dist.abs() as usize);
    } else {
        return ">".repeat(dist as usize);
    }
}

/// Get minimal number of action to change current rune.
fn get_letter_dist(wanted_letter: char, rune: char) -> i32 {
    let rune_pos = VALUES.iter().position(|&x| x == rune).unwrap();
    let wanted_pos = VALUES.iter().position(|&x| x == wanted_letter).unwrap();
    let total_len = VALUES.len();
    return get_mod_dist(rune_pos as i32, wanted_pos as i32, total_len as i32);
}

/// get minimal distance between a and b modulus m
fn get_mod_dist(a: i32, b: i32, m: i32) -> i32 {
    let a = a % m; // ensure range
    let b = b % m;
    let f = (b - a + 2 * m) % m; // ensure positivity
    let g = (a - b + 2 * m) % m;
    if g < f {
        return -g;
    } else {
        return f;
    }
}

/// Return index of rock to activate to minimize action
fn get_minimal_rune_idx(wanted_letter: char, forest: &[char], cur_idx: usize) -> usize {
    let mut min_idx = cur_idx;
    let mut min_op = 30;
    let mut letter_dist = 0;
    let mut idx_dist = 0;
    let forest_len = forest.len();
    for idx in 0..forest_len {
        idx_dist = get_mod_dist(cur_idx as i32, idx as i32, forest_len as i32).abs();
        letter_dist = get_letter_dist(wanted_letter, forest[idx]);
        if (idx_dist + letter_dist) < min_op {
            min_op = idx_dist + letter_dist;
            min_idx = idx;
        }
    }
    return min_idx;
}

fn get_status(forest: &Vec<char>, position: &usize) {
    let s: String = forest.iter().collect();
    eprintln!("{}", s);
    let mut c: Vec<char> = Vec::new();
    for _ in 0..*position {
        c.push(' ')
    }
    c[*position] = '↑';
    let s: String = c.iter().collect();
    eprintln!("{}", s);
}
