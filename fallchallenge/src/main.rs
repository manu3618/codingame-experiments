// https://www.codingame.com/ide/challenge/fall-challenge-2023
use itertools::iproduct;
// use itertools::Itertools;
use rand::Rng;
use std::collections::HashMap;
use std::collections::HashSet;
use std::io;
use std::str::FromStr;

macro_rules! parse_input {
    ($x:expr, $t:ident) => {
        $x.trim().parse::<$t>().unwrap()
    };
}

#[derive(Debug, Default)]
enum Phase {
    /// trying to capture a creature
    Capturing,
    /// trying to explore as much space as possible
    Exploring,
    #[default]
    Diving,
    /// trying to save points
    Surfacing,
    Debug,
}

#[derive(Debug, Default)]
struct Drone {
    id: u8,
    phase: Phase,
    position: (u32, u32),
    emergency: u8,
    battery: u32,
    scanned_unsaved_creature: HashSet<u8>,
}

impl Drone {
    fn creature_distance(&self, other: &Creature) -> u32 {
        distance(self.position, other.position)
    }
    fn distance(&self, position: (u32, u32)) -> u32 {
        distance(self.position, position)
    }
    fn get_command(
        &mut self,
        creatures: &HashMap<u8, Creature>,
        grid: &mut Vec<(u32, u32)>,
    ) -> String {
        eprintln!("getting command for {} ({:?})", &self.id, &self.phase);
        // let light = match nearest {
        //     Some(cid) => {
        //         let position = creatures[&cid].position;
        //         match self.distance(position) {
        //             0..=2000 => 1,
        //             _ => rand::thread_rng().gen_range(0..=1),
        //         }
        //     }
        //     _ => rand::thread_rng().gen_range(0..=1),
        // };
        dbg!(self.scanned_unsaved_creature.len());
        if self.scanned_unsaved_creature.len() > 3 {
            self.phase = Phase::Surfacing;
        }
        if self.position.1 < 500 {
            self.scanned_unsaved_creature.clear();
            self.phase = Phase::Exploring;
        }
        let mut light = rand::thread_rng().gen_range(0..=1);
        if self.battery < 10 {
            light = 0;
        }
        match &self.phase {
            Phase::Capturing => {
                if let Some((a, b)) = self.get_exploration_move(grid) {
                    if self.distance((a, b)) < 2000 {
                        light = 1;
                    }
                    format!("MOVE {a} {b} {light}")
                } else {
                    self.phase = Phase::Diving;
                    self.get_command(creatures, grid)
                }
            }
            Phase::Exploring => {
                dbg!(grid.len());
                if let Some((a, b)) = self.get_exploration_move(grid) {
                    format!("MOVE {a} {b} {light}")
                } else {
                    self.phase = Phase::Capturing;
                    self.get_command(creatures, grid)
                }
            }
            Phase::Diving => {
                if self.position.1 > 9500 {
                    self.phase = Phase::Exploring;
                }
                format!("WAIT {light}")
            }
            Phase::Surfacing => format!("MOVE {} 0 0", self.position.0),
            Phase::Debug => format!("MOVE {} {} 0", self.position.0, self.position.1 + 10),
        }
    }

    fn get_exploration_move(&self, grid: &mut Vec<(u32, u32)>) -> Option<(u32, u32)> {
        let threshold = 2000;
        if grid.is_empty() {
            None
        } else if let Some(p) = &grid
            .clone()
            .iter()
            .map(|a| (a, distance(self.position, *a)))
            .min_by_key(|e| e.1)
        {
            if p.1 < threshold {
                grid.retain(|a| distance(self.position, *a) > threshold);
            };
            Some(*p.0)
        } else {
            None
        }
    }
    fn combine(&mut self, other: &Drone) {
        self.position = other.position;
        self.emergency = other.emergency;
        self.battery = other.battery;
    }
}

impl PartialEq for Drone {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

#[derive(Debug)]
struct ParseDroneError(String);

impl FromStr for Drone {
    type Err = ParseDroneError;
    fn from_str(s: &str) -> Result<Drone, ParseDroneError> {
        let fields = s.trim().split(' ').collect::<Vec<_>>();
        if fields.len() != 5 {
            return Err(ParseDroneError(s.into()));
        }
        let mut drone = Drone::default();
        if let Ok(id) = str::parse(fields[0]) {
            drone.id = id;
        } else {
            return Err(ParseDroneError(s.into()));
        }
        if let Ok(x) = str::parse(fields[1]) {
            if let Ok(y) = str::parse(fields[2]) {
                drone.position = (x, y);
            } else {
                return Err(ParseDroneError(s.into()));
            }
        } else {
            return Err(ParseDroneError(s.into()));
        }
        if let Ok(emergency) = str::parse(fields[3]) {
            drone.emergency = emergency;
        } else {
            return Err(ParseDroneError(s.into()));
        }
        if let Ok(battery) = str::parse(fields[4]) {
            drone.battery = battery;
        } else {
            return Err(ParseDroneError(s.into()));
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
impl PartialEq for Creature {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
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
    drones: Vec<Drone>,
    /// all scanned cretures, including those already saved
    creatures: HashSet<u8>,
}

impl Player {
    /// get of nearest creature.
    /// Returns Some(drone id, creature id)
    fn get_nearest_unscanned_creature(
        &self,
        creatures: &HashMap<u8, Creature>,
    ) -> Option<(u8, u8)> {
        get_nearest_drone_creature(
            //self.drones.iter().map(|d| d),
            self.drones.iter(),
            creatures.values(),
            &self.creatures,
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
        let mut nearest =
            get_nearest_drone_creature(self.drones.iter(), creatures.values(), &self.creatures);

        while let Some(n) = nearest {
            if n.2 > threshold {
                return Some((n.0, n.1));
            } else {
                // dbg!(&creatures[&n.1]);
                if let Some(c) = creatures.get_mut(&n.1) {
                    c.position = (
                        rand::thread_rng().gen_range(1..=10000),
                        rand::thread_rng().gen_range(1..=10000),
                    );
                }
                // dbg!(&creatures[&n.1]);
                nearest = get_nearest_drone_creature(
                    // self.drones.iter().map(|d| d),
                    self.drones.iter(),
                    creatures.values(),
                    &self.creatures,
                );
            }
            if max_iter < 1 {
                return None;
            }
            max_iter -= 1;
        }
        None
    }

    /// update drone if contained by the player, otherwiseadd it.
    fn update_drone(&mut self, drone: Drone) {
        // dbg!(&self.drones);
        let index = &self.drones.iter().position(|d| d == &drone);
        match index {
            Some(idx) => {
                let old = self.drones.get_mut(*idx).unwrap();
                old.combine(&drone);
            }
            None => self.drones.push(drone),
        }
    }
}

fn get_nearest_drone_creature<'a, D, C>(
    drones: D,
    creatures: C,
    excluded: &HashSet<u8>,
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

fn get_exploration_grid() -> Vec<(u32, u32)> {
    let h_step = 850;
    let v_step = h_step + h_step / 2;
    let max_h = 10000;
    let max_v = 10000;
    let mut grid: Vec<(u32, u32)> =
        iproduct!((0..max_h).step_by(h_step), (v_step..max_v).step_by(v_step))
            .map(|(a, b)| (a as u32, b as u32))
            .collect::<Vec<_>>();

    // left transitions
    grid.extend(
        (0..max_v)
            .step_by(2 * v_step) // 1 line in 2
            .map(|y| (0, (y as u32 + v_step as u32 / 2))), //shifted by v_step / 2
    );

    // right transitions
    grid.extend(
        (0..max_v)
            .step_by(2 * v_step) // 1 line in 2
            .map(|y| (max_h as u32, (y as u32 + 3 * v_step as u32 / 2))),
    );

    grid
}

fn parse_game_input(me: &mut Player, foe: &mut Player, creatures: &mut HashMap<u8, Creature>) {
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    me.score = parse_input!(input_line, u32);
    // dbg!(format!("me.score {input_line}"));
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    foe.score = parse_input!(input_line, u32);
    //dbg!(format!("foe.score {input_line}"));
    me.creatures.clear();
    foe.creatures.clear();
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let my_scan_count = parse_input!(input_line, usize);
    // dbg!(format!("my scan count {input_line}"));
    for _ in 0..my_scan_count {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        me.creatures.insert(parse_input!(input_line, u8));
        //   dbg!(format!("creature id {input_line}"));
    }
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let foe_scan_count = parse_input!(input_line, usize);
    //dbg!(format!("foe scan count {input_line}"));
    for _ in 0..foe_scan_count {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        foe.creatures.insert(parse_input!(input_line, u8));
        //dbg!(format!("creature id {input_line}"));
    }
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let my_drone_count = parse_input!(input_line, usize);
    //dbg!(format!("my drone count {input_line}"));
    for _ in 0..my_drone_count {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let drone: Drone = input_line.parse().unwrap();
        me.update_drone(drone);
        //dbg!(format!("drone {input_line}"));
    }

    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let foe_drone_count = parse_input!(input_line, usize);
    //dbg!(format!("foe drone count {input_line}"));
    for _ in 0..foe_drone_count {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let drone: Drone = input_line.parse().unwrap();
        foe.update_drone(drone);
        //dbg!(format!("drone {input_line}"));
    }
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let drone_scan_count = parse_input!(input_line, usize);
    //dbg!(format!("my drone count {input_line}"));
    for _ in 0..drone_scan_count {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        // dbg!(format!("drone id, creature id {input_line}"));
        let line = input_line.trim().split(' ').collect::<Vec<_>>();
        let drone_id = line[0].parse::<u8>().unwrap();
        let creature_id = line[0].parse::<u8>().unwrap();
        if let Some(drone) = me.drones.iter_mut().find(|d| d.id == drone_id) {
            drone.scanned_unsaved_creature.insert(creature_id);
        }
        // dbg!(&me.drones);
        if let Some(drone) = foe.drones.iter_mut().find(|d| d.id == drone_id) {
            drone.scanned_unsaved_creature.insert(creature_id);
        }
    }
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let visible_creature_count = parse_input!(input_line, i32);
    // dbg!(format!("visible creature count {input_line}"));
    for _ in 0..visible_creature_count as usize {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let creature: Creature = input_line.parse().unwrap();
        dbg!(format!("creature {input_line}"));
        let cid = creature.id;
        creatures
            .entry(cid)
            .and_modify(|e| {
                e.combine(&creature);
            })
            .or_insert(creature);
        // dbg!(&creatures[&cid]);
    }
    dbg!("end of parsing");
}

/**
 * Score points by scanning valuable fish faster than your opponent.
 **/
fn main() {
    let mut exploration_grid = get_exploration_grid();
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
    for loop_number in 0..200 {
        eprintln!("beginning of loop {loop_number}");
        parse_game_input(&mut me, &mut foe, &mut creatures);
        // To ignore
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let radar_blip_count = parse_input!(input_line, usize);
        dbg!(radar_blip_count);
        for _i in 0..radar_blip_count {
            let mut input_line = String::new();
            io::stdin().read_line(&mut input_line).unwrap();
            let inputs = input_line.split(' ').collect::<Vec<_>>();
            let _drone_id = parse_input!(inputs[0], i32);
            let _creature_id = parse_input!(inputs[1], i32);
            let _radar = inputs[2].trim().to_string();
            dbg!(&input_line);
        }

        eprintln!("beginning of commands for loop {loop_number}");
        for d in me.drones.iter_mut() {
            if loop_number > 185 {
                d.phase = Phase::Surfacing;
            }
            if rand::thread_rng().gen_range(0..=40) == 0 {
                d.phase = Phase::Surfacing;
            }
            // let nearest = me
            //     .get_update_nearest_creature(&mut creatures)
            //     .map(|(_, n)| n);

            // Write an action using println!("message...");
            // To debug: eprintln!("Debug message...");
            // println!("WAIT 1"); // MOVE <x> <y> <light (1|0)> | WAIT <light (1|0)>
            println!("{}", d.get_command(&creatures, &mut exploration_grid))
        }
        eprintln!("end of loop {loop_number}");
    }
}
