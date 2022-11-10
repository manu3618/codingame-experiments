// https://www.codingame.com/ide/puzzle/vox-codei-episode-1
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
    for i in 0..height as usize {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let map_row = input_line.trim_matches('\n').to_string(); // one line of the firewall grid
    }

    // game loop
    loop {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let inputs = input_line.split(" ").collect::<Vec<_>>();
        let rounds = parse_input!(inputs[0], i32); // number of rounds left before the end of the game
        let bombs = parse_input!(inputs[1], i32); // number of bombs left

        // Write an action using println!("message...");
        // To debug: eprintln!("Debug message...");

        println!("3 0");
    }
}
