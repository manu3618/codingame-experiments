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
    let bitmap = uncompress(image, w as usize, h as usize);
    let classes = classify_rows(bitmap);
    eprintln!("{:?}", classes);
    eprintln!("{:?}", classes.len());

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
            continue;
        }
        if c.trim() == "B" {
            fill_char = 'B';
            continue;
        }
        length = c.trim().parse().unwrap();
        for _ in 0..length {
            image[cur_row][cur_col] = fill_char;
            cur_col = cur_col + 1;
            if cur_col == width {
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
    return vec![image];
}

fn recognize_note(image: Vec<Vec<char>>) -> String {
    /// analyze single note
    // TODO
    return "AQ".to_string();
}

fn classify_rows(image: Vec<Vec<char>>) -> Vec<String> {
    let mut classes: Vec<String> = Vec::new();
    for col in 0..image[0].len() {
        let mut row: Vec<char> = (0..image.len()).map(|x| image[x][col]).collect();
        // eprintln!("{:?}", &row);
        classes.push(classify_row(row));
        // eprintln!("{:?}", classes);
    }
    return classes;
}

fn classify_row(row: Vec<char>) -> String {
    /// indicate type of row (either lines, note, or other)
    if row.iter().all(|&x| x == 'W') {
        return "b".to_string(); // blank
    }
    match detect_lines(row) {
        None => return "n".to_string(),    // note
        Some(x) => return "i".to_string(), // interlines
    }
}
fn detect_lines(row: Vec<char>) -> Option<Vec<usize>> {
    /// Given a row, return the first row of each interline
    /// Return None if unable to detet lines
    let mut blacks: Vec<usize> = Vec::new();
    for idx in 0..row.len() {
        if row[idx] == 'B' {
            blacks.push(idx)
        }
    }
    let mut line_width = 0_usize;
    for w in 0..row.len() {
        if row[blacks[0] + w] == 'W' {
            line_width = w;
            break;
        }
    }
    let mut interline = 0_usize;
    for idx in (blacks[0] + line_width)..row.len() {
        if row[idx] == 'B' {
            if idx - (blacks[0] + line_width) < 8 {
                // not enougn interline
                return None;
            }
            if idx - (blacks[0] + line_width) < (4 * line_width) {
                // not enougn interline
                return None;
            }
            interline = idx - blacks[0];
            break;
        }
    }
    let mut line_start: Vec<usize> = (0..5).map(|x| blacks[0] + x * interline).collect();
    let mut lines: Vec<usize> = Vec::new(); // lines that must be black
    for w in 0..line_width {
        for elt in line_start.iter() {
            lines.push(elt + w)
        }
    }
    for &x in &lines {
        if row[x] == 'W' {
            return None;
        }
    }
    for idx in 0..(lines.iter().max().copied().unwrap() + interline) {
        if lines.contains(&idx) {
            continue;
        }
        if row[idx] == 'B' {
            return None;
        }
    }
    return Some(line_start);
}
