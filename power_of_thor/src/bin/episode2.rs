// https://www.codingame.com/ide/puzzle/power-of-thor-episode-2
use std::io;

macro_rules! parse_input {
    ($x:expr, $t:ident) => {
        $x.trim().parse::<$t>().unwrap()
    };
}

// TODO: cf  https://github.com/rust-lang/rfcs/pull/1546#issuecomment-1493672251

#[derive(Debug)]
struct Thor {
    coord: Coord,
    strikes: i32,
}

#[derive(Debug, Copy, Clone)]
struct Giant {
    coord: Coord,
}

#[derive(Debug, Copy, Clone)]
struct Coord {
    x: i32,
    y: i32,
}

impl Coord {
    /// Get distance to other
    fn dist(&self, other: &Self) -> i32 {
        i32::max((other.x - self.x).abs(), (other.y - self.y).abs())
    }

    /// Get direction to other
    fn dir(&self, other: &Coord) -> String {
        let mut dir = String::from("");
        match other.y - self.y {
            0 => {}
            i32::MIN..=0 => dir.push('N'),
            1..=i32::MAX => dir.push('S'),
        }
        match other.x - self.x {
            0 => {}
            i32::MIN..=0 => dir.push('W'),
            1..=i32::MAX => dir.push('E'),
        }
        dir
    }

    /// Update current coordinates given the movement
    fn move_towards(&mut self, movement: &str) {
        if movement.contains('N') {
            self.y -= 1;
        }
        if movement.contains('S') {
            self.y += 1;
        }
        if movement.contains('E') {
            self.x += 1;
        }
        if movement.contains('W') {
            self.x -= 1;
        }
    }

    /// Move toward specific coordinates
    fn move_to(&mut self, target: &Coord) {
        self.move_towards(&self.dir(target));
    }
}

impl Thor {
    /// l1 distance to the giant
    fn dist_giant(&self, other: &Giant) -> i32 {
        self.coord.dist(&other.coord)
    }

    /// distance to the nearest giant. default to i32::MAX
    fn min_dist(&self, giants: &[Giant]) -> i32 {
        giants
            .iter()
            .map(|x| self.dist_giant(x))
            .reduce(i32::min)
            .unwrap_or(i32::MAX)
    }
    fn dir(&self, dst: &Coord) -> String {
        self.coord.dir(dst)
    }
    /// Update current coordinates given the movement
    fn move_towards(&mut self, movement: &str) {
        self.coord.move_towards(movement)
    }
}

/// barycenter of nearest giants
fn barycenter(giants: &Vec<Coord>) -> Option<Coord> {
    if let Some(giant) = giants.iter().copied().reduce(|x, g| Coord {
        x: g.x + x.x,
        y: g.y + x.y,
    }) {
        Some(Coord {
            x: giant.x / giants.len() as i32,
            y: giant.y / giants.len() as i32,
        })
    } else {
        None
    }
}

/// run simulation to know if the sequence of movement is succesful
fn simulate(thor: &mut Thor, giants: &mut Vec<Giant>, actions: Vec<String>) -> Result<(), String> {
    for action in actions.iter() {
        let act = action.as_str();
        match act {
            "STRIKE" => {
                // update strikes
                thor.strikes -= 1;
                if thor.strikes <= 0 {
                    return Err("Not enough strikes left".into());
                }
                // remove giants
                let _ = &mut giants.retain(|&giant| thor.dist_giant(&giant) > 2);
            }
            "N" | "S" | "E" | "W" | "NE" | "SE" | "SW" | "NW" => thor.move_towards(act),
            _ => unreachable!(),
        }

        move_giants(giants, thor);

        // check victory
        if giants.is_empty() {
            return Ok(());
        }
        if thor.min_dist(giants) < 1 {
            return Err("Reached by a giant".into());
        }
    }
    Err("Not finished".into())
}

fn get_naive_sequence(thor: &mut Thor, giants: &mut Vec<Giant>) -> Vec<String> {
    let mut seq = Vec::new();
    let mut trigger_dist = 2;
    loop {
        // check victory
        if giants.is_empty() || thor.min_dist(giants) < 1 {
            break;
        }

        let dist = thor.min_dist(&giants);
        if dist <= trigger_dist {
            seq.push("STRIKE".into());
            move_giants(giants, thor);
            continue;
        }
        if let Some(dst) = barycenter(&giants.iter().map(|g| g.coord).collect()) {
            let dir = thor.dir(&dst);
            eprintln!("Debug message {:?} {:?} -> {:?}", dir, &thor, &dst);
            thor.move_towards(&dir);
            if !dir.is_empty() {
                seq.push(String::from(&dir));
                trigger_dist = 2;
                move_giants(giants, thor);
                continue;
            }
        } else {
            seq.push("WAIT".into());
            trigger_dist = 1;
            move_giants(giants, thor);
            continue;
        }
        seq.push("WAIT".into());
        move_giants(giants, thor);
    }
    seq
}

fn move_giants(giants: &mut Vec<Giant>, thor: &Thor) {
    for giant in &mut *giants {
        giant.coord.move_to(&thor.coord);
    }
}

fn main() {
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let inputs = input_line.split(' ').collect::<Vec<_>>();
    let tx = parse_input!(inputs[0], i32); // Thor coordinates
    let ty = parse_input!(inputs[1], i32); // Thor coordinates
    let mut thor = Thor {
        coord: Coord { x: tx, y: ty },
        strikes: 0,
    };
    let mut giants: Vec<Giant> = Vec::new();
    let mut trigger_dist = 2;

    let _ = simulate(&mut thor, &mut giants, vec!["WAIT".into()]); // XXX

    // game loop
    loop {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let inputs = input_line.split(' ').collect::<Vec<_>>();
        let _h = parse_input!(inputs[0], i32); // the remaining number of hammer strikes.
        let n = parse_input!(inputs[1], i32); // the number of giants which are still present on the map.
        giants.truncate(0);
        for _ in 0..n as usize {
            let mut input_line = String::new();
            io::stdin().read_line(&mut input_line).unwrap();
            let inputs = input_line.split(' ').collect::<Vec<_>>();
            let x = parse_input!(inputs[0], i32);
            let y = parse_input!(inputs[1], i32);
            eprintln!("Debug message giants: {:?}, {:?}", &x, &y);
            giants.push(Giant {
                coord: Coord { x, y },
            });
        }
        let dist = thor.min_dist(&giants);
        if dist <= trigger_dist {
            println!("STRIKE");
            continue;
        }
        if let Some(dst) = barycenter(&giants.iter().map(|g| g.coord).collect()) {
            let dir = thor.dir(&dst);
            eprintln!("Debug message {:?} {:?} -> {:?}", dir, &thor, &dst);
            thor.move_towards(&dir);
            if !dir.is_empty() {
                println!("{}", dir);
                trigger_dist = 2;
                continue;
            }
        } else {
            println!("WAIT");
            trigger_dist = 1;
            continue;
        }

        println!("WAIT");
    }
}
