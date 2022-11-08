// https://www.codingame.com/ide/puzzle/music-scores
use std::collections::HashSet;
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
    let indexes = get_note_index_image(&bitmap);

    println!("{}", recognize_notes(&bitmap, &indexes));
}

/// Transform description in bitmap image
fn uncompress(description: String, width: usize, height: usize) -> Vec<Vec<char>> {
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

/// get starting and
fn get_note_index_image(image: &Vec<Vec<char>>) -> Vec<Vec<usize>> {
    let classes = classify_rows(image.clone());
    let mut notes: Vec<Vec<usize>> = Vec::new();

    let mut idx = 0_usize;
    let mut start = 0_usize;
    let mut end = 0_usize;

    while idx < image[0].len() {
        start = find_next_start(&classes, idx);
        end = find_next_end(&classes, start);
        idx = end;
        notes.push(vec![start, end]);
    }
    return notes;
}

/// return index of next interline just before note
fn find_next_start(classes: &Vec<char>, start: usize) -> usize {
    for idx in start..(classes.len() - 1) {
        if classes[idx] == 'i' && classes[idx + 1] == 'n' {
            return idx;
        }
    }
    return classes.len();
}

/// return index of next interline just after note
fn find_next_end(classes: &Vec<char>, start: usize) -> usize {
    for idx in start..classes.len() {
        if classes[idx - 1] == 'n' && classes[idx] == 'i' {
            return idx;
        }
    }
    return classes.len();
}

fn recognize_notes(image: &Vec<Vec<char>>, notes: &Vec<Vec<usize>>) -> String {
    let mut result = "".to_string();
    for note in notes {
        let rec = recognize_note(image, note[0], note[1]);
        if rec.len() > 1 {
            result.push_str(" ");
            result.push_str(&rec);
        }
    }
    result.remove(0); // remove leading space
    return result;
}

/// analyze single note
/// the image start at start and ends at end
fn recognize_note(image: &Vec<Vec<char>>, start: usize, end: usize) -> String {
    if end - start <= 2 {
        return "".to_string();
    }
    // let mut row: Vec<char> = (0..image.len()).map(|x| image[x][start]).collect();
    let mut row: Vec<char> = (0..image.len()).map(|x| image[x][start - 1]).collect();
    let mut line_start = detect_lines(&row).unwrap();
    line_start.push(line_start[4] + (line_start[4] - line_start[3])); // last optinal interline
    line_start.insert(0, 0);
    line_start.push(row.len());
    let mut lines: Vec<usize> = Vec::new();
    for idx in 0..row.len() {
        if row[idx] == 'B' {
            lines.push(idx)
        }
    }

    let mut col = start + (end - start) / 4;
    row = (0..image.len()).map(|x| image[x][col]).collect();

    let mut anorm: Vec<usize> = Vec::new();
    for idx in 0..row.len() {
        if (lines.contains(&idx) && row[idx] == 'W') || (!lines.contains(&idx) && row[idx] == 'B') {
            anorm.push(idx)
        }
    }

    let mut anom_pos: HashSet<usize> = HashSet::new();
    for ano in anorm {
        for idx in 0..(line_start.len() - 1) {
            if ano > line_start[idx] && ano < line_start[idx + 1] {
                anom_pos.insert(idx);
            }
        }
    }
    eprintln!("anomalies positions: {:?}", anom_pos);

    let mut result = "".to_string();
    result.push_str(&get_letter(&anom_pos)); // XXX

    let mut col = start + (end - start) / 2;
    row = (0..image.len()).map(|x| image[x][col]).collect();

    result.push_str(&get_color(&anom_pos, &line_start, &row));
    // std::process::exit(0); // XXX
    eprintln!(
        "{}{}",
        get_letter(&anom_pos),
        get_color(&anom_pos, &line_start, &row)
    );

    return result;
}

fn get_letter(anomalies_pos: &HashSet<usize>) -> String {
    if anomalies_pos.len() == 1 {
        match anomalies_pos.iter().min().unwrap() {
            0 => return "G".to_string(),
            1 => return "E".to_string(),
            2 => return "C".to_string(),
            3 => return "A".to_string(),
            4 => return "F".to_string(),
            5 => return "D".to_string(),
            _ => return ".".to_string(),
        }
    } else {
        match anomalies_pos.iter().min().unwrap() {
            0 => return "F".to_string(),
            1 => return "D".to_string(),
            2 => return "B".to_string(),
            3 => return "G".to_string(),
            4 => return "E".to_string(),
            5 => return "C".to_string(),
            _ => return ".".to_string(),
        }
    }
}

fn get_color(anomalies_pos: &HashSet<usize>, line_start: &Vec<usize>, row: &Vec<char>) -> String {
    let inter_line_len = (line_start[2] - line_start[1]) / 2;
    let mid_note: usize;
    if anomalies_pos.len() == 1 {
        let mid_note_pos = anomalies_pos.iter().next().unwrap();
        mid_note = line_start[*mid_note_pos] + inter_line_len;
    } else {
        let mid_note_pos = anomalies_pos.iter().max().unwrap();
        mid_note = line_start[*mid_note_pos]
    }
    // {
    //     for idx in 0..row.len() {
    //         let mut mark = "".to_string();
    //         if line_start.contains(&idx) {
    //             mark = "i".to_string();
    //         }
    //         if idx == mid_note {
    //             mark.push_str("<-");
    //         }
    //         eprintln!("DEBUG 280 {}\t{} {}", idx, row[idx], mark);
    //     }
    // }
    match row[mid_note] {
        'B' => return "Q".to_string(),
        'W' => return "H".to_string(),
        _ => return ".".to_string(),
    }
}

fn classify_rows(image: Vec<Vec<char>>) -> Vec<char> {
    let mut classes: Vec<char> = Vec::new();
    for col in 0..image[0].len() {
        let mut row: Vec<char> = (0..image.len()).map(|x| image[x][col]).collect();
        classes.push(classify_row(row));
    }
    return classes;
}

fn classify_row(row: Vec<char>) -> char {
    /// indicate type of row (either lines, note, or other)
    if row.iter().all(|&x| x == 'W') {
        return 'b'; // blank
    }
    match detect_lines(&row) {
        None => return 'n',    // note
        Some(x) => return 'i', // interlines
    }
}

fn detect_lines(row: &Vec<char>) -> Option<Vec<usize>> {
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
