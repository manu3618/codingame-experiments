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
    let mut forest: Vec<char> = Vec::new(); // current status
    let mut output_letter = "".to_string();

    // Write an action using println!("message...");
    // To debug: eprintln!("Debug message...");
    for position in 0..letters.len() {
        output_letter += ">";
        let init_value = ' ';
        output_letter += &get_letter(letters[position], &init_value);
        output_letter += ".";
    }

    println!("{}", output_letter);
}

/// Get part of string needed to obtain the new letter
/// run is modified in place
fn get_letter(wanted_letter: char, rune: &char) -> String {
    eprintln!("{}", *rune);
    let rune_pos = VALUES.iter().position(|&x| x == *rune).unwrap();
    let wanted_pos = VALUES.iter().position(|&x| x == wanted_letter).unwrap();

    if rune_pos > wanted_pos {
        let diff = rune_pos - wanted_pos;
        return "-".repeat(diff as usize);
    } else {
        let diff = wanted_pos - rune_pos;
        return "+".repeat(diff as usize);
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
