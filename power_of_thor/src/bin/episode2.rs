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
    x: i32,
    y: i32,
}

#[derive(Debug, Copy, Clone)]
struct Giant {
    x: i32,
    y: i32,
}

trait Point {
    /// Get distance to other
    fn dist(&self, other: &Self) -> i32;
    /// Get direction to other
    fn dir(&self, other: &Giant) -> String;
    /// Get minimal distance to all others
    // fn min_dist(&self, &others: Vec<dyn Point>) -> i32;
    /// Update current coordinates given the movement
    fn move_towards(&mut self, movement: &str);
}

impl Point for Thor {
    fn dist(&self, other: &Self) -> i32 {
        i32::max((other.x - self.x).abs(), (other.y - self.y).abs())
    }

    fn dir(&self, other: &Giant) -> String {
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
}

impl Thor {
    /// l1 distance to the giant
    fn dist_giant(&self, other: &Giant) -> i32 {
        i32::max((other.x - self.x).abs(), (other.y - self.y).abs())
    }

    /// distance to the nearest giant. default to i32::MAX
    fn min_dist(&self, giants: &Vec<Giant>) -> i32 {
        giants
            .iter()
            .map(|x| self.dist_giant(x))
            .reduce(i32::min)
            .unwrap_or(i32::MAX)
    }
}

/// l1 distance between 2 points
fn dist(a: (i32, i32), b: (i32, i32)) -> i32 {
    i32::max((a.0 - b.0).abs(), (a.1 - b.1).abs())
}

/// minimal distance between giants and thor
fn min_dist(giants: &Vec<(i32, i32)>, thor: (i32, i32)) -> i32 {
    giants
        .iter()
        .map(|x| dist(*x, thor))
        .reduce(i32::min)
        .unwrap_or(i32::MAX)
}

/// barycenter of nearest giants
fn barycenter(giants: &Vec<Giant>) -> Option<Giant> {
    if let Some(giant) = giants.iter().copied().reduce(|x, g| Giant {
        x: g.x + x.x,
        y: g.y + x.y,
    }) {
        Some(Giant {
            x: giant.x / giants.len() as i32,
            y: giant.y / giants.len() as i32,
        })
    } else {
        None
    }
}

fn main() {
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let inputs = input_line.split(' ').collect::<Vec<_>>();
    let tx = parse_input!(inputs[0], i32); // Thor coordinates
    let ty = parse_input!(inputs[1], i32); // Thor coordinates
    let mut thor = Thor { x: tx, y: ty };
    let mut giants: Vec<Giant> = Vec::new();
    let mut trigger_dist = 2;

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
            giants.push(Giant { x: x, y: y });
        }
        let dist = thor.min_dist(&giants);
        if dist <= trigger_dist {
            println!("STRIKE");
            continue;
        }
        if let Some(dst) = barycenter(&giants) {
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

        // Write an action using println!("message...");
        // To debug: eprintln!("Debug message...");

        // The movement or action to be carried out: WAIT STRIKE N NE E SE S SW W or N
        println!("WAIT");
    }
}
