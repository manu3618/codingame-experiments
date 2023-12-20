// https://www.codingame.com/ide/challenge/fall-challenge-2023
use itertools::iproduct;
use std::collections::HashMap;
use std::io;
use std::str::FromStr;

macro_rules! parse_input {
    ($x:expr, $t:ident) => {
        $x.trim().parse::<$t>().unwrap()
    };
}

#[derive(Debug, Default)]
struct Drone {
    id: u8,
    position: (u32, u32),
    emergency: u8,
    battery: u32,
}

impl Drone {
    fn creature_distance(&self, other: &Creature) -> u32 {
        distance(self.position, other.position)
    }
}

#[derive(Debug)]
struct ParseDroneError;

impl FromStr for Drone {
    type Err = ParseDroneError;
    fn from_str(s: &str) -> Result<Drone, ParseDroneError> {
        let fields = s.trim().split(" ").collect::<Vec<_>>();
        if fields.len() != 5 {
            return Err(ParseDroneError);
        }
        let mut drone = Drone::default();
        if let Ok(id) = str::parse(fields[0]) {
            drone.id = id;
        } else {
            return Err(ParseDroneError);
        }
        if let Ok(x) = str::parse(fields[1]) {
            if let Ok(y) = str::parse(fields[2]) {
                drone.position = (x, y);
            } else {
                return Err(ParseDroneError);
            }
        } else {
            return Err(ParseDroneError);
        }
        if let Ok(emergency) = str::parse(fields[3]) {
            drone.emergency = emergency;
        } else {
            return Err(ParseDroneError);
        }
        if let Ok(battery) = str::parse(fields[4]) {
            drone.battery = battery;
        } else {
            return Err(ParseDroneError);
        }
        Ok(drone)
    }
}

#[derive(Debug, Default)]
struct Creature {
    id: u8,
    creature_type: u8,
    color: u8,
    position: (u32, u32),
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
        if fields.len() != 3 && fields.len() != 5 {
            return Err(ParseCreatureError);
        }
        let mut creature = Creature::default();
        if let Ok(id) = str::parse(fields[0]) {
            creature.id = id;
        } else {
            return Err(ParseCreatureError);
        }
        if fields.len() == 3 {
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
        }
        if fields.len() == 5 {
            if let Ok(x) = str::parse(fields[1]) {
                if let Ok(y) = str::parse(fields[2]) {
                    creature.position = (x, y);
                } else {
                    return Err(ParseCreatureError);
                }
            } else {
                return Err(ParseCreatureError);
            }
            if let Ok(x) = str::parse(fields[3]) {
                if let Ok(y) = str::parse(fields[4]) {
                    creature.velocity = (x, y);
                } else {
                    return Err(ParseCreatureError);
                }
            } else {
                return Err(ParseCreatureError);
            }
        }
        Ok(creature)
    }
}

fn distance(a: (u32, u32), b: (u32, u32)) -> u32 {
    ((a.0 as i32 - b.0 as i32).pow(2) + (a.1 as i32 - b.1 as i32).pow(2)) as u32
}

#[derive(Debug, Default)]
struct Player {
    score: u32,
    drones: HashMap<u8, Drone>,
    /// creature ids
    creatures: Vec<u8>,
}

impl Player {
    /// get position of nearest creature.
    fn get_nearest_unscanned_creature(
        &self,
        creatures: &HashMap<u8, Creature>,
    ) -> Option<(i32, i32)> {
        // creatures.filter_map(|c| if &self.creatures.contains(c.id){None} else {distance(&self.position)}})
        None
    }
}

fn parse_game_input(me: &mut Player, foe: &mut Player, creatures: &mut HashMap<u8, Creature>) {
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    me.score = parse_input!(input_line, u32);
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    foe.score = parse_input!(input_line, u32);
    me.creatures.truncate(0);
    foe.creatures.truncate(0);
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let my_scan_count = parse_input!(input_line, usize);
    for _ in 0..my_scan_count {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        me.creatures.push(parse_input!(input_line, u8));
    }
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let foe_scan_count = parse_input!(input_line, usize);
    for _ in 0..foe_scan_count {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        foe.creatures.push(parse_input!(input_line, u8));
    }
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let my_drone_count = parse_input!(input_line, usize);
    for _ in 0..my_drone_count {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let drone: Drone = input_line.parse().unwrap();
        me.drones.insert(drone.id, drone);
    }

    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let foe_drone_count = parse_input!(input_line, usize);
    for _ in 0..foe_drone_count as usize {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let drone: Drone = input_line.parse().unwrap();
        foe.drones.insert(drone.id, drone);
    }
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let drone_scan_count = parse_input!(input_line, i32);
    for _ in 0..drone_scan_count as usize {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        foe.creatures.push(parse_input!(input_line, u8));
    }
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let visible_creature_count = parse_input!(input_line, i32);
    for _ in 0..visible_creature_count as usize {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let creature: Creature = input_line.parse().unwrap();
        creatures.insert(creature.id, creature);
    }
    dbg!(&me);
    dbg!(&foe);
    dbg!(&creatures);
}

/**
 * Score points by scanning valuable fish faster than your opponent.
 **/
fn main() {
    let mut input_line = String::new();
    let mut creatures: HashMap<u8, Creature> = HashMap::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let creature_count = parse_input!(input_line, usize);
    for _ in 0..creature_count {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let creature: Creature = input_line.parse().unwrap();
        creatures.insert(creature.id, creature);
    }
    let mut me = Player::default();
    let mut foe = Player::default();

    // game loop
    loop {
        parse_game_input(&mut me, &mut foe, &mut creatures);
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
        for i in 0..me.drones.len() {
            // Write an action using println!("message...");
            // To debug: eprintln!("Debug message...");

            println!("WAIT 1"); // MOVE <x> <y> <light (1|0)> | WAIT <light (1|0)>
        }
    }
}
