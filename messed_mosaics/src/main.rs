// https://www.codingame.com/ide/puzzle/messed-up-mosaics
use std::io;

macro_rules! parse_input {
    ($x:expr, $t:ident) => {
        $x.trim().parse::<$t>().unwrap()
    };
}

fn get_mismatch_idx(line: String, pattern: String) -> Option<usize> {
    for idx in 0..line.len() {
        if !is_line_ok(line[idx..].into(), pattern.clone())
            && is_line_ok(line[(idx + 1)..].into(), pattern.clone())
        {
            return Some(idx);
        }
    }
    None
}

fn is_line_ok(line: String, pattern: String) -> bool {
    let expected = pattern.repeat((line.len() / pattern.len()) + 2);
    expected.contains(&line)
}

/**
 * Auto-generated code below aims at helping you parse
 * the standard input according to the problem statement.
 **/
fn main() {
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let n = parse_input!(input_line, i32);
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let pattern = input_line.trim_matches('\n').to_string();
    for i in 0..n as usize {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let row = input_line.trim_matches('\n').to_string();

        if let Some(idx) = get_mismatch_idx(row, pattern.clone()) {
            println!("({},{})", idx, i);
            return;
        }
    }
    unreachable!()
}
