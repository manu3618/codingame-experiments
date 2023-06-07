// https://www.codingame.com/ide/puzzle/power-of-thor-episode-2

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
    let tx = parse_input!(inputs[0], i32);
    let ty = parse_input!(inputs[1], i32);

    // game loop
    loop {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let inputs = input_line.split(" ").collect::<Vec<_>>();
        let h = parse_input!(inputs[0], i32); // the remaining number of hammer strikes.
        let n = parse_input!(inputs[1], i32); // the number of giants which are still present on the map.
        for i in 0..n as usize {
            let mut input_line = String::new();
            io::stdin().read_line(&mut input_line).unwrap();
            let inputs = input_line.split(" ").collect::<Vec<_>>();
            let x = parse_input!(inputs[0], i32);
            let y = parse_input!(inputs[1], i32);
        }

        // Write an action using println!("message...");
        // To debug: eprintln!("Debug message...");

        // The movement or action to be carried out: WAIT STRIKE N NE E SE S SW W or N
        println!("WAIT");
    }
}
