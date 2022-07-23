// https://codingame.com/training/medium/there-is-no-spoon-episode-1
use std::io;

macro_rules! parse_input {
    ($x:expr, $t:ident) => {
        $x.trim().parse::<$t>().unwrap()
    };
}

/**
 * Don't let the machines win. You are humanity's last hope...
 **/
fn main() {
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let width = parse_input!(input_line, i32); // the number of cells on the X axis
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let height = parse_input!(input_line, i32); // the number of cells on the Y axis

    eprintln!("Debug message... width {}, height {}", width, height);
    let mut cells: Vec<Vec<char>> = Vec::new();
    cells = vec![vec!['0'; width as usize]; height as usize];

    for h_idx in 0..height as usize {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let line = input_line.trim_matches('\n').to_string(); // width characters, each either 0 or .
        eprintln!("Debug message... line {}", line);
        for w_idx in 0..width as usize {
            cells[h_idx][w_idx] = line.chars().nth(w_idx).unwrap();
        }
    }

    eprintln!("Debug message... cells {:?}", cells);

    let mut x1 = 0; // current coordonates
    let mut y1 = 0;
    let mut x2 = 0; // neighboor on the right side
    let mut y2 = 0;
    let mut x3 = 0; // neighboor behind
    let mut y3 = 0;

    for h_idx in 0..height as usize {
        for w_idx in 0..width as usize {
            if cells[h_idx][w_idx] == '.' {
                continue;
            }
            x1 = w_idx as i32;
            y1 = h_idx as i32;
            eprintln!(
                "Debug message... cell ({}, {}) {}",
                x1, y1, cells[h_idx][w_idx]
            );

            // right side
            y2 = y1;
            if w_idx == (width - 1) as usize {
                x2 = -1;
            } else {
                // for..else
                let mut x_idx = None;
                for x2 in (w_idx + 1) as i32..width {
                    if cells[h_idx][x2 as usize] == '0' {
                        x_idx = Some(x2);
                        break;
                    }
                }
                x2 = match x_idx {
                    None => -1,
                    Some(x) => x,
                };
            }
            if x2 == -1 {
                y2 = -1
            }

            // behind
            x3 = x1;
            if h_idx == (height - 1) as usize {
                y3 = -1;
            } else {
                let mut y_idx = None;
                for y3 in (h_idx + 1) as i32..height {
                    if cells[y3 as usize][w_idx] == '0' {
                        y_idx = Some(y3);
                        break;
                    }
                }
                y3 = match y_idx {
                    None => -1,
                    Some(x) => x,
                }
            };
            if y3 == -1 {
                x3 = -1
            };

            eprintln!("printing {} {} {} {} {} {}", x1, y1, x2, y2, x3, y3);
            println!("{} {} {} {} {} {}", x1, y1, x2, y2, x3, y3)
        }
    }
}
