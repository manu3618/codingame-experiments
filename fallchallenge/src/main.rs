// https://www.codingame.com/ide/challenge/fall-challenge-2023

use std::collections::HashMap;
use std::io;
use std::str::FromStr;

macro_rules! parse_input {
    ($x:expr, $t:ident) => {
        $x.trim().parse::<$t>().unwrap()
    };
}

#[derive(Debug)]
struct Drone {
    position: (u32, u32),
    battery: u32,
}

#[derive(Debug, Default)]
struct Creature {
    id: u8,
    creature_type: u8,
    color: u8,
    position: (i32, i32),
    velocity: (i32, i32),
}

impl Creature {
    fn set_id(self, id: u8) -> Self {
        Self { id: id, ..self }
    }
    fn set_type(self, creature_type: u8) -> Self {
        Self {
            creature_type: creature_type,
            ..self
        }
    }
    fn set_color(self, color: u8) -> Self {
        Self {
            color: color,
            ..self
        }
    }
}

#[derive(Debug)]
struct ParseCreatureError;

impl FromStr for Creature {
    type Err = ParseCreatureError;
    fn from_str(s: &str) -> Result<Creature, ParseCreatureError> {
        let fields = s.trim().split(" ").collect::<Vec<_>>();
        dbg!(&fields);
        if fields.len() != 3 {
            return Err(ParseCreatureError);
        }
        let mut creature = Creature::default();
        if let Ok(id) = str::parse(fields[0]) {
            creature.id = id;
        } else {
            return Err(ParseCreatureError);
        }
        if let Ok(t) = str::parse(fields[1]) {
            creature.creature_type = t;
        } else {
            return Err(ParseCreatureError);
        }
        if let Ok(color) = str::parse(fields[2]) {
            creature.color = color;
        } else {
            return Err(ParseCreatureError);
        }
        Ok(creature)
    }
}

/**
 * Score points by scanning valuable fish faster than your opponent.
 **/
fn main() {
    let mut input_line = String::new();
    let mut creatures: HashMap<u8, Creature> = HashMap::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let creature_count = parse_input!(input_line, i32);
    for _ in 0..creature_count as usize {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let creature: Creature = input_line.parse().unwrap();
        creatures.insert(creature.id, creature);
    }
    dbg!(&creatures);

    // game loop
    loop {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let my_score = parse_input!(input_line, i32);
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let foe_score = parse_input!(input_line, i32);
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let my_scan_count = parse_input!(input_line, i32);
        for i in 0..my_scan_count as usize {
            let mut input_line = String::new();
            io::stdin().read_line(&mut input_line).unwrap();
            let creature_id = parse_input!(input_line, i32);
        }
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let foe_scan_count = parse_input!(input_line, i32);
        for i in 0..foe_scan_count as usize {
            let mut input_line = String::new();
            io::stdin().read_line(&mut input_line).unwrap();
            let creature_id = parse_input!(input_line, i32);
        }
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let my_drone_count = parse_input!(input_line, i32);
        for i in 0..my_drone_count as usize {
            let mut input_line = String::new();
            io::stdin().read_line(&mut input_line).unwrap();
            let inputs = input_line.split(" ").collect::<Vec<_>>();
            let drone_id = parse_input!(inputs[0], i32);
            let drone_x = parse_input!(inputs[1], i32);
            let drone_y = parse_input!(inputs[2], i32);
            let emergency = parse_input!(inputs[3], i32);
            let battery = parse_input!(inputs[4], i32);
        }
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let foe_drone_count = parse_input!(input_line, i32);
        for i in 0..foe_drone_count as usize {
            let mut input_line = String::new();
            io::stdin().read_line(&mut input_line).unwrap();
            let inputs = input_line.split(" ").collect::<Vec<_>>();
            let drone_id = parse_input!(inputs[0], i32);
            let drone_x = parse_input!(inputs[1], i32);
            let drone_y = parse_input!(inputs[2], i32);
            let emergency = parse_input!(inputs[3], i32);
            let battery = parse_input!(inputs[4], i32);
        }
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let drone_scan_count = parse_input!(input_line, i32);
        for i in 0..drone_scan_count as usize {
            let mut input_line = String::new();
            io::stdin().read_line(&mut input_line).unwrap();
            let inputs = input_line.split(" ").collect::<Vec<_>>();
            let drone_id = parse_input!(inputs[0], i32);
            let creature_id = parse_input!(inputs[1], i32);
        }
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let visible_creature_count = parse_input!(input_line, i32);
        for i in 0..visible_creature_count as usize {
            let mut input_line = String::new();
            io::stdin().read_line(&mut input_line).unwrap();
            let inputs = input_line.split(" ").collect::<Vec<_>>();
            let creature_id = parse_input!(inputs[0], i32);
            let creature_x = parse_input!(inputs[1], i32);
            let creature_y = parse_input!(inputs[2], i32);
            let creature_vx = parse_input!(inputs[3], i32);
            let creature_vy = parse_input!(inputs[4], i32);
        }
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let radar_blip_count = parse_input!(input_line, i32);
        for i in 0..radar_blip_count as usize {
            let mut input_line = String::new();
            io::stdin().read_line(&mut input_line).unwrap();
            let inputs = input_line.split(" ").collect::<Vec<_>>();
            let drone_id = parse_input!(inputs[0], i32);
            let creature_id = parse_input!(inputs[1], i32);
            let radar = inputs[2].trim().to_string();
        }
        for i in 0..my_drone_count as usize {
            // Write an action using println!("message...");
            // To debug: eprintln!("Debug message...");

            println!("WAIT 1"); // MOVE <x> <y> <light (1|0)> | WAIT <light (1|0)>
        }
    }
}
