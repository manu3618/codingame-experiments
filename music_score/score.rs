// https://www.codingame.com/ide/puzzle/music-scores
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
    let w = parse_input!(inputs[0], i32);
    let h = parse_input!(inputs[1], i32);
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let image = input_line.trim_matches('\n').to_string();

    // Write an answer using println!("message...");
    // To debug: eprintln!("Debug message...");
    uncompress(image, w as usize, h as usize);

    println!("AQ DH");
}

fn uncompress(description: String, width: usize, height: usize) -> Vec<Vec<char>> {
    /// Transform description in bitmap image
    let mut image = vec![vec![' '; width as usize]; height as usize];
    let mut cur_row = 0_usize;
    let mut cur_col = 0_usize;
    let mut length: usize;
    let mut fill_char = ' ';
    for c in description.split_whitespace() {
        if c.trim() == "W" {
            fill_char = 'W';
            eprintln!("{}\t {}\t {}", cur_row, cur_col, fill_char);
            continue;
        }
        if c.trim() == "B" {
            fill_char = 'B';
            eprintln!("{}\t {}\t {}", cur_row, cur_col, fill_char);
            continue;
        }
        length = c.trim().parse().unwrap();
        for _ in 0..length {
            image[cur_row][cur_col] = fill_char;
            cur_col = cur_col + 1;
            if cur_col == width {
                // eprintln!("{}\t{}", cur_row, image[cur_row].iter().collect::<String>());
                cur_col = 0;
                cur_row = cur_row + 1;
            }
        }
    }
    return image;
}

fn split_image(image: Vec<Vec<char>>) -> Vec<Vec<Vec<char>>> {
    /// split image in individual notes
    // TODO
    return [image];
}

fn recognize_note(image: Vec<Vec<char>>) -> str {
    /// analyze single note
    // TODO
    return "AQ";
}
