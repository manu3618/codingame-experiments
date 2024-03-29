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

#[derive(Debug, Default, PartialEq, Eq)]
enum Phase {
    /// trying to capture a creature
    #[default]
    Capturing,
    /// trying to explore as much space as possible
    Exploring,
    Diving,
    /// trying to save points
    Surfacing,
    /// Hide from monster
    Hiding,
    Debug,
}

impl Phase {
    fn get_random() -> Self {
        match rand::thread_rng().gen_range(0..=3) {
            0 => Self::Capturing,
            1 => Self::Exploring,
            2 => Self::Diving,
            3 => Self::Surfacing,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug)]
struct Drone {
    id: u8,
    phase: Phase,
    position: (u32, u32),
    emergency: u8,
    battery: u32,
    scanned_unsaved_creature: HashSet<u8>,
    /// position of creatures
    /// ```
    /// [
    ///     [ TL: Vec<u8>, TR: Vec<u8> ],
    ///     [ BL: Vec<u8>, BR: Vec<u8> ],
    /// ]
    /// ```
    radar: Vec<Vec<Vec<u8>>>,
    monster_radar: Vec<Vec<Vec<u8>>>,
    /// creatures to scan
    assigned_creature: HashSet<u8>,
}

impl Drone {
    fn creature_distance(&self, other: &Creature) -> u32 {
        distance(self.position, other.position)
    }

    fn distance(&self, position: (u32, u32)) -> u32 {
        distance(self.position, position)
    }

    fn change_phase(&mut self) {
        if self.scanned_unsaved_creature.is_empty() {
            self.phase = match rand::thread_rng().gen_range(0..=2) {
                0 => Phase::Capturing,
                1 => Phase::Exploring,
                2 => Phase::Diving,
                _ => unreachable!(),
            };
        } else if self.phase != Phase::Surfacing {
            self.phase = Phase::get_random();
        }
    }

    fn combine(&mut self, other: &Drone) {
        self.position = other.position;
        self.emergency = other.emergency;
        self.battery = other.battery;
    }

    fn clear_radar(&mut self) {
        self.radar = vec![vec![Vec::new(), Vec::new()], vec![Vec::new(), Vec::new()]]
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
        if self.scanned_unsaved_creature.len() > 5 {
            self.phase = Phase::Surfacing;
        }
        if self.position.1 < 500 {
            self.scanned_unsaved_creature.clear();
            self.change_phase();
        }
        if self.phase == Phase::Diving && self.position.1 > 9900 {
            self.change_phase();
        }
        let mut light = rand::thread_rng().gen_range(0..=1);
        if self.battery < 10 {
            light = 0;
        } else if self.battery > 25 {
            light = 1;
        }
        match &self.phase {
            Phase::Capturing => {
                if let Some((a, b)) = self.get_capture_move() {
                    // if self.distance((a, b)) < 20 {
                    //     light = 1;
                    // }
                    format!("MOVE {a} {b} {light}")
                } else {
                    self.assigned_creature.extend(0..12);
                    self.change_phase();
                    self.get_command(creatures, grid)
                }
            }
            Phase::Exploring => {
                self.change_phase();
                if let Some((a, b)) = self.get_exploration_move(grid) {
                    format!("MOVE {a} {b} {light}")
                } else {
                    self.change_phase();
                    self.get_command(creatures, grid)
                }
            }
            Phase::Diving => {
                if self.position.1 > 9500 {
                    self.change_phase();
                }
                format!("WAIT {light}")
            }
            Phase::Surfacing => format!("MOVE {} 0 0", self.position.0),
            Phase::Debug => format!("MOVE {} {} 0", self.position.0, self.position.1 + 10),
            Phase::Hiding => {
                if self.scanned_unsaved_creature.is_empty() {
                    self.change_phase();
                } else {
                    self.phase = Phase::Surfacing;
                }
                let (a, b) = self.get_hiding_move(creatures);
                format!("MOVE {a} {b} 0")
            }
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

    fn get_capture_move(&mut self) -> Option<(u32, u32)> {
        // TODO
        // find nearest creature on radar
        // find sector
        let (idx, length) = self
            .radar
            .iter()
            .flatten()
            .map(|a| a.len())
            .enumerate()
            .max_by_key(|a| a.1)
            .expect("radar not empty");
        // compute displacement
        if length == 0 {
            return None;
        }
        let step = 850;
        let r = match idx {
            0 | 1 => {
                if self.position.1 < step {
                    (self.position.0, 0)
                } else {
                    (self.position.0, self.position.1 - step)
                }
            }
            2 | 3 => {
                if self.position.1 + step > 10000 {
                    (self.position.0, 10000)
                } else {
                    (self.position.0, self.position.1 + step)
                }
            }
            _ => unreachable!(),
        };
        match idx {
            0 | 2 => {
                if r.0 < step {
                    Some((0, r.1))
                } else {
                    Some((r.0 - step, r.1))
                }
            }
            1 | 3 => {
                if r.0 + step > 10000 {
                    Some((10000, r.1))
                } else {
                    Some((r.0 + step, r.1))
                }
            }
            _ => unreachable!(),
        }
    }

    fn get_sorted_monster_coords(&self, creature: &HashMap<u8, Creature>) -> Vec<(u32, u32)> {
        let mut monster_coords = creature
            .values()
            .filter(|c| c.id > 12)
            .map(|c| c.position)
            .map(|p| (p, self.distance(p)))
            .collect::<Vec<_>>();
        monster_coords.sort_by_key(|a| a.1);
        monster_coords.iter().map(|a| a.0).collect::<Vec<_>>()
    }

    fn get_monster_distance(&self, creature: &HashMap<u8, Creature>) -> u32 {
        let nearest = creature
            .values()
            .filter(|c| c.id > 12)
            .map(|c| c.position)
            .map(|p| (p, self.distance(p)))
            .min_by_key(|a| a.1);
        nearest.unwrap().1
    }

    /// get movement to hide from monsters
    fn get_hiding_move(&self, creatures: &HashMap<u8, Creature>) -> (u32, u32) {
        let monster_coords = &self.get_sorted_monster_coords(&creatures);
        let nearest = monster_coords[0];
        let direction = (
            self.position.0 as i32 - nearest.0 as i32,
            self.position.1 as i32 - nearest.1 as i32,
        );
        (
            (self.position.0 as i32 + direction.0).min(10000).max(0) as u32,
            (self.position.1 as i32 + direction.1).min(10000).max(0) as u32,
        )
    }
}
impl Default for Drone {
    fn default() -> Self {
        Self {
            id: 0,
            phase: Phase::default(),
            position: (0, 0),
            emergency: 0,
            battery: 0,
            scanned_unsaved_creature: HashSet::new(),
            radar: vec![vec![Vec::new(), Vec::new()], vec![Vec::new(), Vec::new()]],
            monster_radar: vec![vec![Vec::new(), Vec::new()], vec![Vec::new(), Vec::new()]],
            assigned_creature: HashSet::with_capacity(12),
        }
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
    creature_type: i8,
    color: i8,
    position: (u32, u32),
    velocity: (i32, i32),
}

impl Creature {
    /// combine self id, type and color with other position and velocity
    fn combine(&self, other: &Creature) -> Self {
        Self {
            id: self.id,
            creature_type: self.creature_type,
            color: self.color,
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
            position: (5000, 100000),
            // rand::thread_rng().gen_range(1..=10000),
            // rand::thread_rng().gen_range(1..=10000),
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
            // dbg!(&creature);
        }
        Ok(creature)
    }
}

fn distance(a: (u32, u32), b: (u32, u32)) -> u32 {
    ((a.0 as i32 - b.0 as i32).pow(2) + (a.1 as i32 - b.1 as i32).pow(2)) as u32
}

fn normalize<T>(a: (T, T)) -> (T, T)
where
    T: From<i32>,
    i32: From<T>,
{
    let b: (i32, i32) = (a.0.into(), a.1.into());
    let l = 10000;
    let n = ((b.0.pow(2) + b.1.pow(2)) as f32).sqrt() as i32;
    ((b.0 * l / n).into(), (b.1 * l / n).into())
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

    /// assign creatures to scan to drones
    fn assign_creatures(&mut self) {
        let dlen = self.drones.len();
        for a in 0..12 {
            self.drones
                .get_mut(a % dlen)
                .unwrap()
                .assigned_creature
                .insert(a as u8);
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
    let h_step = 1250;
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
        // dbg!(format!("creature id {input_line}"));
    }
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let foe_scan_count = parse_input!(input_line, usize);
    // dbg!(format!("foe scan count {input_line}"));
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
        let creature_id = line[1].parse::<u8>().unwrap();
        if let Some(drone) = me.drones.iter_mut().find(|d| d.id == drone_id) {
            drone.scanned_unsaved_creature.insert(creature_id);
            me.creatures.insert(creature_id);
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
        // dbg!(format!("creature {input_line}"));
        let cid = creature.id;
        creatures
            .entry(cid)
            .and_modify(|e| *e = e.combine(&creature))
            .or_insert(creature);
        // dbg!(&creatures[&cid]);
    }
    for drone in me.drones.iter_mut() {
        drone.clear_radar();
        if drone.position.1 < 500 {
            for creature in drone.scanned_unsaved_creature.drain() {
                me.creatures.insert(creature);
            }
        }
    }
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let radar_blip_count = parse_input!(input_line, usize);
    // dbg!(radar_blip_count);
    for _ in 0..radar_blip_count {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let inputs = input_line.split(' ').collect::<Vec<_>>();
        let drone_id = parse_input!(inputs[0], u8);
        let creature_id = parse_input!(inputs[1], u8);
        if me.creatures.contains(&creature_id) {
            continue;
        }
        if let Some(drone) = me.drones.iter_mut().find(|d| d.id == drone_id) {
            if drone.scanned_unsaved_creature.contains(&creature_id) {
                continue;
            }
            if !drone.assigned_creature.contains(&creature_id) && creature_id <= 12 {
                continue;
            }
        }
        let radar = inputs[2].trim();
        if let Some(drone) = me.drones.iter_mut().find(|d| d.id == drone_id) {
            if creatures[&creature_id].color == -1 {
                match radar {
                    "TL" => drone.monster_radar[0][0].push(creature_id),
                    "TR" => drone.monster_radar[0][1].push(creature_id),
                    "BL" => drone.monster_radar[1][0].push(creature_id),
                    "BR" => drone.monster_radar[1][1].push(creature_id),
                    _ => unreachable!(),
                }
            } else {
                match radar {
                    "TL" => drone.radar[0][0].push(creature_id),
                    "TR" => drone.radar[0][1].push(creature_id),
                    "BL" => drone.radar[1][0].push(creature_id),
                    "BR" => drone.radar[1][1].push(creature_id),
                    _ => unreachable!(),
                }
            }
        }
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
        if loop_number == 0 {
            me.assign_creatures();
        }
        eprintln!("beginning of commands for loop {loop_number}");
        let drones_coord = me
            .drones
            .iter()
            .map(|d| (d.id, d.position))
            .collect::<Vec<_>>();

        let unsaved = me
            .drones
            .iter()
            .map(|d| d.scanned_unsaved_creature.iter())
            .flatten()
            .collect::<Vec<_>>()
            .iter()
            .map(|&d| *d)
            .collect::<HashSet<u8>>();

        for d in me.drones.iter_mut() {
            dbg!(d.get_monster_distance(&creatures));
            if d.get_monster_distance(&creatures) < 1_000_000 {
                d.phase = Phase::Hiding;
            }
            // change phase if drone are too close from each other
            if drones_coord
                .clone()
                .iter()
                .any(|&c| d.id != c.0 && d.distance(c.1) < 500000)
                && d.phase != Phase::Surfacing
            {
                dbg!(&drones_coord);
                d.change_phase();
            }

            // randomly change phase
            if rand::thread_rng().gen_range(0..=80) == 0 {
                d.change_phase();
            }

            // surface if end is near
            if loop_number > 185 {
                d.phase = Phase::Surfacing;
            }

            if (&unsaved | &me.creatures).len() == 12 {
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
