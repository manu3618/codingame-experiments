// https://www.codingame.com/ide/puzzle/tower-dereference
use itertools::enumerate;
use std::fmt;
use std::io;
use std::str::FromStr;

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

impl From<&str> for Side {
    fn from(s: &str) -> Self {
        match s {
            "0" => Self::Left,
            "1" => Self::Right,
            _ => panic!("unable to parse Side"),
        }
    }
}

#[derive(Default, Debug)]
struct Attacker {
    id: usize,
    owner: Side,
    // coordinates: (usize, usize),
    coordinates: (f32, f32),
    hit_points: u32,
    max_hit_points: u32,
    current_speed: u32,
    max_speed: u32,
    slow_time: u32,
    bounty: u32,
}

impl Attacker {
    fn from_stdin() -> Self {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let inputs = input_line.trim().split(" ").collect::<Vec<_>>();
        dbg!(&inputs);
        debug_assert!(inputs.len() == 9);
        Self {
            id: inputs[0].parse().unwrap(),
            owner: inputs[1].into(),
            coordinates: (inputs[2].parse().unwrap(), inputs[3].parse().unwrap()),
            hit_points: inputs[4].parse().unwrap(),
            max_hit_points: inputs[5].parse().unwrap(),
            current_speed: inputs[6].parse().unwrap(),
            max_speed: inputs[7].parse().unwrap(),
            slow_time: inputs[8].parse().unwrap(),
            bounty: inputs[9].parse().unwrap(),
        }
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

#[derive(Debug)]
struct TowerParseError;

impl fmt::Display for TowerType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Gun => "GUNTOWER",
            Self::Fire => "FIRETOWER",
            Self::Glue => "GLUETOWER",
            Self::Heal => "HEALTOWER",
        };
        write!(f, "{}", s)
    }
}

impl FromStr for TowerType {
    type Err = TowerParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "GUNTOWER" => Ok(Self::Gun),
            "FIRETOWER" => Ok(Self::Fire),
            "GLUETOWER" => Ok(Self::Glue),
            "HEALTOWER" => Ok(Self::Heal),
            _ => Err(TowerParseError),
        }
    }
}

impl From<&str> for TowerType {
    fn from(s: &str) -> Self {
        Self::from_str(s).unwrap()
    }
}

#[derive(Default, Debug)]
enum UpgradeType {
    #[default]
    Damage,
    Range,
    Reload,
}

impl fmt::Display for UpgradeType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Damage => "DAMAGE",
            Self::Range => "RANGE",
            Self::Reload => "RELOAD",
        };
        write!(f, "{}", s)
    }
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

    /// upgrade level of the tower
    damage_level: u8,
    range_level: u8,
    reload_level: u8,
}

impl Tower {
    fn from_stdin() -> Self {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let inputs = input_line.trim().split(" ").collect::<Vec<_>>();
        dbg!(&inputs);
        debug_assert!(inputs.len() == 9);

        Self {
            id: inputs[1].parse().unwrap(),
            tower_type: inputs[0].into(),
            owner: inputs[2].into(),
            coordinates: (inputs[3].parse().unwrap(), inputs[4].parse().unwrap()),
            damage: inputs[5].parse().unwrap(),
            range: inputs[6].parse().unwrap(),
            cooldown: inputs[8].parse().unwrap(),
            ..Default::default()
        }
    }

    fn upgradable(&self, up_type: UpgradeType) -> bool {
        match up_type {
            UpgradeType::Damage => self.damage_level < 3,
            UpgradeType::Range => self.range_level < 3,
            UpgradeType::Reload => self.reload_level < 3,
        }
    }
}

/**
 * Survive the attack waves
 **/
fn main() {
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let player_id = parse_input!(input_line, u8);
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
        let tower_count = parse_input!(input_line, usize);
        for i in 0..tower_count {
            let tower = Tower::from_stdin();
            dbg!(tower);
        }
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let attacker_count = parse_input!(input_line, usize);
        for i in 0..attacker_count {
            let attacker = Attacker::from_stdin();
            dbg!(attacker);
        }

        // Write an action using println!("message...");
        // To debug: eprintln!("Debug message...");

        println!("BUILD 5 5 GUNTOWER"); // BUILD x y TOWER | UPGRADE id PROPERTY
    }
}
