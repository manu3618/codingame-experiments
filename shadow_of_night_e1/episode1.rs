// https://www.codingame.com/ide/puzzle/shadows-of-the-knight-episode-1
use std::cmp;
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
    let w = parse_input!(inputs[0], i32); // width of the building.
    let h = parse_input!(inputs[1], i32); // height of the building.
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let n = parse_input!(input_line, i32); // maximum number of turns before game over.
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let inputs = input_line.split(" ").collect::<Vec<_>>();
    let mut x0 = parse_input!(inputs[0], i32);
    let mut y0 = parse_input!(inputs[1], i32);
    let mut x_min = 0;
    let mut x_max = w;
    let mut y_min = 0;
    let mut y_max = h;

    // game loop
    loop {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let bomb_dir = input_line.trim().to_string(); // the direction of the bombs from batman's current location (U, UR, R, DR, D, DL, L or UL)

        // Write an action using println!("message...");
        // To debug: eprintln!("Debug message...");

        eprintln!("Debug message... bomb_dir {}", bomb_dir);
        eprintln!("Debug message...  {}\tx\t {}", x_min, x_max);
        eprintln!("Debug message...  {}\ty\t {}", y_min, y_max);

        if bomb_dir.contains("U") {
            y_max = y0;
            y0 = cmp::max((y_min + y0) / 2, 0);
        }

        if bomb_dir.contains("D") {
            y_min = y0;
            y0 = cmp::min((y_max + y0) / 2, h - 1);
        }

        if bomb_dir.contains("R") {
            x_min = x0;
            x0 = cmp::min((x_max + x0) / 2, w - 1);
        }
        if bomb_dir.contains("L") {
            x_max = x0;
            x0 = cmp::max((x_min + x0) / 2, 0);
        }

        // the location of the next window Batman should jump to.
        println!("{} {}", x0, y0);
    }
}
