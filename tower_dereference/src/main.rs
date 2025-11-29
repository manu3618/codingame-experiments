// https://www.codingame.com/ide/puzzle/tower-dereference
use std::io;

macro_rules! parse_input {
    ($x:expr, $t:ident) => ($x.trim().parse::<$t>().unwrap())
}

/**
 * Survive the attack waves
 **/
fn main() {
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let player_id = parse_input!(input_line, i32);
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let inputs = input_line.split(" ").collect::<Vec<_>>();
    let width = parse_input!(inputs[0], i32);
    let height = parse_input!(inputs[1], i32);
    for i in 0..height as usize {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let line = input_line.trim_matches('\n').to_string();
    }

    // game loop
    loop {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let inputs = input_line.split(" ").collect::<Vec<_>>();
        let my_money = parse_input!(inputs[0], i32);
        let my_lives = parse_input!(inputs[1], i32);
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let inputs = input_line.split(" ").collect::<Vec<_>>();
        let opponent_money = parse_input!(inputs[0], i32);
        let opponent_lives = parse_input!(inputs[1], i32);
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let tower_count = parse_input!(input_line, i32);
        for i in 0..tower_count as usize {
            let mut input_line = String::new();
            io::stdin().read_line(&mut input_line).unwrap();
            let inputs = input_line.split(" ").collect::<Vec<_>>();
            let tower_type = inputs[0].trim().to_string();
            let tower_id = parse_input!(inputs[1], i32);
            let owner = parse_input!(inputs[2], i32);
            let x = parse_input!(inputs[3], i32);
            let y = parse_input!(inputs[4], i32);
            let damage = parse_input!(inputs[5], i32);
            let attack_range = parse_input!(inputs[6], f64);
            let reload = parse_input!(inputs[7], i32);
            let cool_down = parse_input!(inputs[8], i32);
        }
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let attacker_count = parse_input!(input_line, i32);
        for i in 0..attacker_count as usize {
            let mut input_line = String::new();
            io::stdin().read_line(&mut input_line).unwrap();
            let inputs = input_line.split(" ").collect::<Vec<_>>();
            let attacker_id = parse_input!(inputs[0], i32);
            let owner = parse_input!(inputs[1], i32);
            let x = parse_input!(inputs[2], f64);
            let y = parse_input!(inputs[3], f64);
            let hit_points = parse_input!(inputs[4], i32);
            let max_hit_points = parse_input!(inputs[5], i32);
            let current_speed = parse_input!(inputs[6], f64);
            let max_speed = parse_input!(inputs[7], f64);
            let slow_time = parse_input!(inputs[8], i32);
            let bounty = parse_input!(inputs[9], i32);
        }

        // Write an action using println!("message...");
        // To debug: eprintln!("Debug message...");

        println!("BUILD 5 5 GUNTOWER"); // BUILD x y TOWER | UPGRADE id PROPERTY
    }
}
