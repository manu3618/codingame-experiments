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

    // Write an action using println!("message...");
    // To debug: eprintln!("Debug message...");
    for position in 0..letters.len() {
        let idx = position % forest.len();
        output_letter += ">";
        let init_value = forest[idx];
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

/// Get minimal number of action to change current rune.
fn get_letter_dist(wanted_letter: char, rune: char) -> i32 {
    let rune_pos = VALUES.iter().position(|&x| x == rune).unwrap();
    let wanted_pos = VALUES.iter().position(|&x| x == wanted_letter).unwrap();
    let total_len = VALUES.len() as i32;
    let forward_dist = (total_len + wanted_pos as i32 - rune_pos as i32) % total_len;
    let backward_dist = (total_len + rune_pos as i32 - wanted_pos as i32) % total_len;
    if backward_dist < forward_dist {
        let backward_dist = -backward_dist;
        return backward_dist;
    } else {
        return forward_dist;
    }
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
