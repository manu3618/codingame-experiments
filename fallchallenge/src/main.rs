// https://www.codingame.com/ide/challenge/fall-challenge-2023
use itertools::iproduct;
use rand::Rng;
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
    fn distance(&self, position: (u32, u32)) -> u32 {
        distance(self.position, position)
    }
}

#[derive(Debug)]
struct ParseDroneError;

impl FromStr for Drone {
    type Err = ParseDroneError;
    fn from_str(s: &str) -> Result<Drone, ParseDroneError> {
        let fields = s.trim().split(' ').collect::<Vec<_>>();
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

#[derive(Debug)]
struct Creature {
    id: u8,
    creature_type: u8,
    color: u8,
    position: (u32, u32),
    velocity: (i32, i32),
}

impl Creature {
    /// combine self id, type and color with other position and velocity
    fn combine(&self, other: &Creature) -> Self {
        Self {
            id: self.id,
            creature_type: self.creature_type.max(other.creature_type),
            color: self.color.max(other.color),
            position: other.position,
            velocity: other.velocity,
        }
    }
}

impl Default for Creature {
    fn default() -> Self {
        Self {
            id: 0,
            creature_type: 0,
            color: 0,
            position: (
                rand::thread_rng().gen_range(1..=10000),
                rand::thread_rng().gen_range(1..=10000),
            ),
            velocity: (0, 0),
        }
    }
}

#[derive(Debug)]
struct ParseCreatureError;

impl FromStr for Creature {
    type Err = ParseCreatureError;
    fn from_str(s: &str) -> Result<Creature, ParseCreatureError> {
        let fields = s.trim().split(' ').collect::<Vec<_>>();
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
    /// get of nearest creature.
    /// Returns Some(drone id, creature id)
    fn get_nearest_unscanned_creature(
        &self,
        creatures: &HashMap<u8, Creature>,
    ) -> Option<(u8, u8)> {
        get_nearest_drone_creature(
            self.drones.values(),
            creatures.values(),
            self.creatures.clone(),
        )
        .map(|(a, b, _)| (a, b))
    }

    /// Get nearest creature.
    /// If creature is in scanning range but not scannable, it must have moved.
    /// Randomly update those creatures.
    fn get_update_nearest_creature(
        &self,
        creatures: &mut HashMap<u8, Creature>,
    ) -> Option<(u8, u8)> {
        let threshold = 500;
        let mut max_iter = 10;
        let mut nearest = get_nearest_drone_creature(
            self.drones.values(),
            creatures.values(),
            self.creatures.clone(),
        );

        while let Some(n) = nearest {
            if n.2 > threshold {
                return Some((n.0, n.1));
            } else {
                dbg!(&creatures[&n.1]);
                if let Some(c) = creatures.get_mut(&n.1) {
                    c.position = (
                        rand::thread_rng().gen_range(1..=10000),
                        rand::thread_rng().gen_range(1..=10000),
                    );
                }
                dbg!(&creatures[&n.1]);
                nearest = get_nearest_drone_creature(
                    self.drones.values(),
                    creatures.values(),
                    self.creatures.clone(),
                );
            }
            if max_iter < 1 {
                return None;
            }
            max_iter -= 1;
        }
        None
    }
}

fn get_nearest_drone_creature<'a, D, C>(
    drones: D,
    creatures: C,
    excluded: Vec<u8>,
) -> Option<(u8, u8, u32)>
where
    D: Iterator<Item = &'a Drone>,
    C: Iterator<Item = &'a Creature>,
    C: Clone,
{
    iproduct!(drones, creatures)
        .filter_map(|(d, c)| {
            if excluded.contains(&c.id) {
                None
            } else {
                Some((d.id, c.id, d.creature_distance(c)))
            }
        })
        .collect::<Vec<_>>()
        .iter()
        .min_by_key(|e| e.2)
        .copied()
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
    for _ in 0..foe_drone_count {
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
        let cid = creature.id;
        creatures
            .entry(cid)
            .and_modify(|e| {
                e.combine(&creature);
            })
            .or_insert(creature);
        // dbg!(&creatures[&cid]);
    }
    // dbg!(&me);
    // dbg!(&foe);
    // dbg!(&creatures);
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
        let nearest = me
            .get_update_nearest_creature(&mut creatures)
            .map(|(_, n)| n);

        // To ignore
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let radar_blip_count = parse_input!(input_line, i32);
        for _i in 0..radar_blip_count as usize {
            let mut input_line = String::new();
            io::stdin().read_line(&mut input_line).unwrap();
            let inputs = input_line.split(' ').collect::<Vec<_>>();
            let _drone_id = parse_input!(inputs[0], i32);
            let _creature_id = parse_input!(inputs[1], i32);
            let _radar = inputs[2].trim().to_string();
        }

        for d in me.drones.values() {
            // Write an action using println!("message...");
            // To debug: eprintln!("Debug message...");
            // println!("WAIT 1"); // MOVE <x> <y> <light (1|0)> | WAIT <light (1|0)>
            match nearest {
                Some(cid) => {
                    let position = creatures[&cid].position;
                    let light = match d.distance(position) {
                        0..=2000 => 1,
                        _ => rand::thread_rng().gen_range(0..=1),
                    };
                    println!("MOVE {} {} {}", position.0, position.1, light);
                }
                _ => {
                    println!("WAIT 1");
                }
            }
        }
    }
}
