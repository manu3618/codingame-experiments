// https://www.codingame.com/ide/puzzle/tower-dereference
use itertools::enumerate;
use itertools::iproduct;
use std::collections::HashMap;
use std::fmt;
use std::io;
use std::ops::Add;
use std::ops::Mul;
use std::ops::Neg;
use std::ops::Sub;
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

    fn get(&self, coords: (usize, usize)) -> Option<char> {
        self.0.get(coords.0)?.get(coords.1).copied()
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

#[derive(Default, Debug, PartialEq, Eq, Clone, Copy)]
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

impl Side {
    fn invert(self) -> Self {
        match self {
            Self::Right => Self::Left,
            Self::Left => Self::Right,
        }
    }
}

#[derive(Default, Debug, Copy, Clone)]
enum Property {
    #[default]
    HitPoint,
    Speed,
    Bounty,
    None,
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
        debug_assert!(inputs.len() == 9);
        Self {
            id: inputs[0].parse().unwrap(),
            owner: inputs[1].into(),
            coordinates: (inputs[3].parse().unwrap(), inputs[2].parse().unwrap()),
            hit_points: inputs[4].parse().unwrap(),
            max_hit_points: inputs[5].parse().unwrap(),
            current_speed: inputs[6].parse().unwrap(),
            max_speed: inputs[7].parse().unwrap(),
            slow_time: inputs[8].parse().unwrap(),
            bounty: inputs[9].parse().unwrap(),
        }
    }

    fn get_property(&self, prop: Property) -> u32 {
        match prop {
            Property::Bounty => self.bounty,
            Property::HitPoint => self.hit_points,
            Property::Speed => self.current_speed,
            Property::None => 1,
        }
    }
}

#[derive(Default, Debug, Clone, Copy, Eq, PartialEq, Hash)]
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

impl TowerType {
    fn get_all() -> impl Iterator<Item = Self> {
        vec![Self::Gun, Self::Fire, Self::Glue, Self::Heal].into_iter()
    }
    fn price(&self) -> usize {
        match &self {
            Self::Gun => 100,
            Self::Fire => 100,
            Self::Glue => 70,
            Self::Heal => 100,
        }
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
    /// screenwise coordinate (tof left, [lines, row])
    coordinates: (usize, usize),
    damage: usize,
    range: f64,

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
        debug_assert!(inputs.len() == 9);

        Self {
            id: inputs[1].parse().unwrap(),
            tower_type: inputs[0].into(),
            owner: inputs[2].into(),
            coordinates: (inputs[4].parse().unwrap(), inputs[3].parse().unwrap()),
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

// TODO:
// * is_command_feasable
// * score_to_place_tower(map, attacker, tower_type)
//   [x] close to paths
//   [ ] ahead of ennemies (attackers)
//   [ ] ahead of ennemy towers (heal)
// * something to upgrade tower
// * get_commands
//

#[derive(Default, Debug)]
struct ScoreMap(Vec<Vec<i32>>);

impl fmt::Display for ScoreMap {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "   {}\n",
            (0..self.0.first().unwrap().len())
                .map(|x| format!("{:>2}", x))
                .collect::<String>()
        )?;
        for (num, v) in enumerate(&self.0) {
            write!(
                f,
                "{:>2} {}\n",
                num,
                v.iter().map(|c| format!("{:>2}", c)).collect::<String>()
            )?
        }
        Ok(())
    }
}

impl Mul<ScoreMap> for i32 {
    type Output = ScoreMap;

    fn mul(self, m: ScoreMap) -> ScoreMap {
        ScoreMap(
            m.0.iter()
                .map(|v| v.iter().map(|x| self * x).collect())
                .collect(),
        )
    }
}

impl Neg for ScoreMap {
    type Output = Self;

    fn neg(self) -> Self::Output {
        -1 * self
    }
}

impl ScoreMap {
    fn from_map(map: &Map) -> Self {
        let mut m = Self::from_map_with_range(&map, 0);
        for r in 1..3 {
            m = m + Self::from_map_with_range(&map, r);
        }
        let (l_num, c_num) = map.size();
        for (l, r) in iproduct!(0..l_num, 0..c_num) {
            if map.get((l, r)) == Some('.') {
                m.0[l][r] = 0;
            }
        }
        m
    }

    fn from_map_with_range(map: &Map, range: usize) -> Self {
        let multiplier = 100;
        let (l_num, c_num) = map.size();
        let mut score = Self(vec![vec![0; c_num]; l_num]);

        for (l, r) in iproduct!(0..l_num, 0..c_num) {
            if map.get((l, r)) == Some('.') {
                for c in score.get_neighbors((l, r), range) {
                    score.0[c.0][c.1] += multiplier;
                }
            }
        }
        score
    }

    fn from_attackers(
        size: (usize, usize),
        attackers: &[Attacker],
        side: Side,
        prop: Property,
        multiplier: i32,
    ) -> Self {
        let range = 2;
        let mut score = Self(vec![vec![0; size.0]; size.1]);
        for a in attackers.iter().filter(|x| x.owner == side) {
            let coords = (a.coordinates.0 as usize, a.coordinates.1 as usize);
            for c in score.get_neighbors(coords, range) {
                score.0[c.0][c.1] += multiplier * a.get_property(prop) as i32;
            }
        }
        score
    }

    fn from_towers(
        size: (usize, usize),
        towers: &[Tower],
        side: Side,
        tower_types: &[TowerType],
        multiplier: i32,
    ) -> Self {
        let mut score = Self(vec![vec![0; size.0]; size.1]);
        for t in towers
            .iter()
            .filter(|x| x.owner == side && tower_types.contains(&x.tower_type))
        {
            let coords = (t.coordinates.0 as usize, t.coordinates.1 as usize);
            for c in score.get_neighbors(coords, 1 + t.range as usize) {
                score.0[c.0][c.1] += multiplier * t.range as i32;
            }
        }
        score
    }

    /// positive values are towards side.
    fn from_side(size: (usize, usize), side: Side) -> Self {
        Self(vec![
            (0..size.1)
                .map(|a| {
                    let m = match side {
                        Side::Left => 1,
                        Side::Right => -1,
                    };
                    m * (a as i32 - (size.1 as i32) / 2)
                })
                .collect::<Vec<_>>();
            size.0
        ])
    }

    fn get(&self, coords: (usize, usize)) -> Option<i32> {
        self.0.get(coords.0)?.get(coords.1).copied()
    }

    fn get_neighbors(&self, coords: (usize, usize), size: usize) -> Vec<(usize, usize)> {
        let col_min = 0.max(coords.0 as i32 - size as i32) as usize;
        let row_min = 0.max(coords.1 as i32 - size as i32) as usize;
        let col_max = self.0.len().min(coords.0 + size);
        let row_max = self.0.first().unwrap().len().min(coords.1 + size);
        iproduct!(col_min..col_max, row_min..row_max)
            .filter(|&a| a != coords)
            .collect()
    }

    fn substract_towers(&mut self, towers: &[Tower]) {
        for t in towers {
            self.0[t.coordinates.0][t.coordinates.1] = 0;
        }
    }

    /// Get preferences to place a tower
    fn tower_preference(
        map: &Map,
        tower_type: TowerType,
        my_side: Side,
        attackers: &[Attacker],
        towers: &[Tower],
    ) -> Self {
        match tower_type {
            TowerType::Gun | TowerType::Fire => {
                Self::from_attackers(
                    map.size(),
                    attackers,
                    my_side.invert(),
                    Property::HitPoint,
                    1,
                ) + Self::from_attackers(
                    map.size(),
                    attackers,
                    my_side.invert(),
                    Property::Bounty,
                    3,
                )
            }
            TowerType::Glue => {
                Self::from_attackers(map.size(), attackers, my_side.invert(), Property::Speed, 5)
                    + Self::from_attackers(
                        map.size(),
                        attackers,
                        my_side.invert(),
                        Property::HitPoint,
                        5,
                    )
            }
            TowerType::Heal => {
                Self::from_attackers(map.size(), attackers, my_side, Property::None, 30)
                    - Self::from_attackers(map.size(), attackers, my_side, Property::HitPoint, 1)
                    + Self::from_towers(
                        map.size(),
                        towers,
                        my_side.invert(),
                        &[TowerType::Fire, TowerType::Gun],
                        1,
                    )
                    + Self::from_side(map.size(), my_side.invert())
            }
        }
    }

    fn get_max(&self) -> i32 {
        *self.0.iter().flatten().max().unwrap()
    }
    fn get_whichmax(&self) -> (usize, usize) {
        let (line_num, line) = (&self.0)
            .into_iter()
            .enumerate()
            .max_by_key(|(_, row)| *row.iter().max().unwrap())
            .unwrap();
        let (col_num, _) = line
            .into_iter()
            .enumerate()
            .max_by_key(|(_, v)| *v)
            .unwrap();
        (line_num, col_num)
    }

    fn get_ordered_map(&self) -> Vec<(usize, usize)> {
        let s = self.size();
        let mut scores = iproduct!(0..s.0, 0..s.1)
            .map(|(a, b)| (self.0[a][b], (a, b)))
            .collect::<Vec<_>>();
        scores.sort_by_key(|(s, _)| *s);
        scores.iter().map(|(_, e)| e).copied().collect::<Vec<_>>()
    }

    fn size(&self) -> (usize, usize) {
        (self.0.len(), self.0[0].len())
    }
}

impl Add for ScoreMap {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        assert_eq!(self.size(), other.size());
        let (l_num, c_num) = self.size();
        let mut r = Self(vec![vec![0; c_num]; l_num]);
        for (col, row) in iproduct!(0..self.0.len(), 0..self.0[0].len()) {
            r.0[col][row] = self.0[col][row] + other.0[col][row]
        }
        r
    }
}

impl Sub for ScoreMap {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        assert_eq!(self.size(), other.size());
        let (l_num, c_num) = self.size();
        let mut r = Self(vec![vec![0; c_num]; l_num]);
        for (col, row) in iproduct!(0..self.0.len(), 0..self.0[0].len()) {
            r.0[col][row] = self.0[col][row] - other.0[col][row]
        }
        r
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
    let tower_types = [
        "GUNTOWER",
        "FIRETOWER",
        "GLUETOWER",
        "HEALTOWER",
        "GUNTOWER",
        "GUNTOWER",
    ];
    let tower_types = tower_types
        .iter()
        .map(|a| a.parse().unwrap())
        .collect::<Vec<TowerType>>();
    let mut tower_types = tower_types.iter().cycle();

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
        let mut towers = Vec::with_capacity(tower_count);
        for _ in 0..tower_count {
            towers.push(Tower::from_stdin());
        }
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let attacker_count = parse_input!(input_line, usize);
        let mut attackers = Vec::with_capacity(attacker_count);
        for _ in 0..attacker_count {
            attackers.push(Attacker::from_stdin());
        }

        // TODO: logic to choose next tower type
        // take decision based on max score for each type
        let tower_prefs = TowerType::get_all()
            .map(|t| {
                (
                    t,
                    ScoreMap::tower_preference(&map, t, me.side, &attackers, &towers),
                )
            })
            .map(|(tower_type, scoremap)| (tower_type, scoremap.get_max()))
            .collect::<Vec<_>>();
        dbg!(&tower_prefs);
        let my_heal = &attackers
            .iter()
            .filter(|a| a.owner == me.side)
            .map(|a| a.hit_points)
            .collect::<Vec<_>>();
        let opponent_heal = &attackers
            .iter()
            .filter(|a| a.owner == opponent.side)
            .map(|a| a.hit_points)
            .collect::<Vec<_>>();
        dbg!(my_heal.iter().sum::<u32>() / my_heal.len() as u32);
        dbg!(opponent_heal.iter().sum::<u32>() / opponent_heal.len() as u32);
        // XXX
        let tower_type = tower_types.next().unwrap().clone();

        // Write an action using println!("message...");
        // To debug: eprintln!("Debug message...");

        // TODO
        // println!("BUILD 5 5 GUNTOWER"); // BUILD x y TOWER | UPGRADE id PROPERTY
        let mut score = if attackers.len() == 0 {
            3 * ScoreMap::from_map(&map) + ScoreMap::from_side(map.size(), me.side)
        } else {
            ScoreMap::tower_preference(&map, tower_type, me.side, &attackers, &towers)
        };
        score.substract_towers(&towers);

        let build_coords = score.get_whichmax();
        if me.money >= tower_type.price() {
            println!("BUILD {} {} {}", build_coords.1, build_coords.0, tower_type);
        } else {
            println!("PASS");
        }
    }
}
