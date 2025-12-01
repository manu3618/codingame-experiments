// https://www.codingame.com/ide/puzzle/tower-dereference
use itertools::enumerate;
use std::fmt;
use std::io;

macro_rules! parse_input {
    ($x:expr, $t:ident) => {
        $x.trim().parse::<$t>().unwrap()
    };
}

#[derive(Default)]
struct Map(Vec<Vec<char>>);

impl fmt::Display for Map {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "  {}\n",
            (0..self.0.first().unwrap().len())
                .map(|x| format!("{}", x % 10))
                .collect::<String>()
        )?;
        for (num, v) in enumerate(&self.0) {
            write!(f, "{} {}\n", num % 10, v.iter().collect::<String>())?
        }
        Ok(())
    }
}

impl Map {
    fn size(&self) -> (usize, usize) {
        (self.0.len(), self.0.first().map_or(0, |elt| elt.len()))
    }
    fn from_stdin() -> Self {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let mut line = input_line.split(" ");
        let (Some(width), Some(height)) = (line.next(), line.next()) else {
            panic!("fail to parse map size")
        };
        let width: usize = width.trim().parse().unwrap();
        let height: usize = height.trim().parse().unwrap();

        let mut map = Vec::new();
        for _ in 0..height {
            input_line.clear();
            io::stdin().read_line(&mut input_line).unwrap();
            map.push(input_line.trim().chars().into_iter().collect::<Vec<_>>());
            debug_assert!(map.last().unwrap().len() == width);
        }
        Self(map.clone())
    }
}

#[derive(Default, Debug)]
struct Player {
    side: Side,
    money: usize,
    live: usize,
}

impl Player {
    fn from_stdin(side: Side) -> Self {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let inputs = input_line.split(" ").collect::<Vec<_>>();
        Self {
            side: side,
            money: inputs[0].trim().parse().unwrap(),
            live: inputs[1].trim().parse().unwrap(),
        }
    }
}

#[derive(Default, Debug)]
enum Side {
    #[default]
    Left,
    Right,
}

#[derive(Default, Debug)]
struct Attacker {
    id: usize,
    owner: Side,
    coordinates: (usize, usize),
    hit_points: u32,
    max_hit_points: u32,
    current_speed: u32,
    max_speed: u32,
    slow_time: u32,
    bounty: u32,
}

impl Attacker {
    fn from_stdin() -> Self {
        todo!()
    }
}

#[derive(Default, Debug)]
enum TowerType {
    #[default]
    Gun,
    Fire,
    Glue,
    Heal,
}

#[derive(Default, Debug)]
struct Tower {
    id: usize,
    tower_type: TowerType,
    owner: Side,
    coordinates: (usize, usize),
    damage: usize,
    range: usize,

    /// number of turns left before being able to fire again
    cooldown: usize,
    damage_level: u8,
    range_level: u8,
    reload_level: u8,
}

impl Tower {
    fn from_stdin() -> Self {
        todo!()
    }
}

/**
 * Survive the attack waves
 **/
fn main() {
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let player_id = parse_input!(input_line, i32);
    let map = Map::from_stdin();
    eprintln!("map \n{}", &map);

    // game loop
    loop {
        let (me, opponent) = if player_id == 0 {
            (
                Player::from_stdin(Side::Left),
                Player::from_stdin(Side::Right),
            )
        } else {
            (
                Player::from_stdin(Side::Right),
                Player::from_stdin(Side::Left),
            )
        };

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
