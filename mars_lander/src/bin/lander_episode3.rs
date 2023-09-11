// https://www.codingame.com/training/expert/mars-lander-episode-3
use std::cmp;
use std::io;
use std::iter::zip;

const MAX_ROTATION: i32 = 16;
const MAX_SPEED: i32 = 60;

macro_rules! parse_input {
    ($x:expr, $t:ident) => {
        $x.trim().parse::<$t>().unwrap()
    };
}

/// compute margin
/// if absolute bvalue of margin is greater than 1, then we can let the lander
/// freefall
/// if the absolute value of margin is lower than one, the full thrust is required
/// to avoid crash
fn z_margin(h_speed: i32, height: i32) -> f64 {
    let max_thrust = 4.0;
    let max_a = max_thrust - 3.711;
    ((height as f64) * max_a) / (2.0 * h_speed as f64)
}

/// PID implementation for rotation
fn get_rotation(h_speed: i32, landing_distance: i32) -> i32 {
    let k0 = 2;
    let k1 = 1;
    let result = -k0 * landing_distance / 100 + k1 * h_speed;
    let max_rotation = 45;
    eprintln!(
        "Debug message... speed {} distance {} result {}",
        h_speed, landing_distance, result
    );
    cmp::max(-max_rotation, cmp::min(result, max_rotation))
}

/// Get possible landing phase given the landoer position and the landig site
fn get_landing_phase(position: (i32, i32), target: (i32, i32)) -> LandingPhase {
    let line = Line {
        p0: position,
        p1: target,
    };
    if let Some((a, _)) = line.get_equation() {
        if a.abs() < 0.25 {
            if position.0 < target.0 {
                return LandingPhase::Horizontal(Direction::Right);
            } else {
                return LandingPhase::Horizontal(Direction::Left);
            }
        }
    }
    return LandingPhase::Direct;
}

#[derive(Debug)]
struct Line {
    p0: (i32, i32),
    p1: (i32, i32),
}

impl Line {
    /// get a and b coefficient so that equation looks like
    /// y = ax + b
    fn get_equation(&self) -> Option<(f64, f64)> {
        if self.p0.1 == self.p1.1 {
            return None;
        }
        let a = (self.p0.0 - self.p1.0) as f64 / (self.p0.1 - self.p1.1) as f64;
        let b = self.p0.1 as f64 - a * self.p0.0 as f64;
        Some((a, b))
    }

    /// get intersection point between 2 lines. a line is defined by 2 points
    fn get_intersection(&self, other: &Line) -> Option<(f64, f64)> {
        if let (Some((a, b)), Some((c, d))) = (self.get_equation(), other.get_equation()) {
            let x = (b - d) / (a - c);
            let y = a * x + b;
            Some((x, y))
        } else {
            None
        }
    }
}

#[derive(Debug)]
struct Terrain {
    land: Vec<(i32, i32)>,
}

impl Terrain {
    /// compute middle of langding site coordinates
    fn get_landing_site(&self) -> (i32, i32) {
        let mut previous = (0, 0);
        for point in &self.land {
            if point.1 == previous.1 {
                return ((previous.0 + point.0) / 2, previous.1);
            }
            previous = (point.0, point.1);
        }
        previous
    }

    /// determine if the direct route between start and target enter the terrain
    fn has_conflict(&self, start: (i32, i32), target: (i32, i32)) -> bool {
        let direct = Line {
            p0: start,
            p1: target,
        };
        let mut previous = (0, 0);
        for point in &self.land {
            let segment = Line {
                p0: previous,
                p1: *point,
            };
            if let Some((x, y)) = direct.get_intersection(&segment) {
                if x > f64::min(previous.0 as f64, point.0 as f64)
                    && x > f64::max(previous.0 as f64, point.0 as f64)
                {
                    eprintln!(
                        "Debug message... collision detected {:?}, {:?}, {:?}",
                        direct,
                        &segment,
                        (x, y)
                    );
                    return true;
                }
            }
        }
        false
    }

    fn get_target(&self, start: (i32, i32)) -> (i32, i32) {
        let landing_site = self.get_landing_site();
        if !self.has_conflict(start, landing_site) {
            return landing_site;
        }
        if landing_site.0 > start.0 {
            (0, start.1)
        } else {
            (7000, start.1)
        }
    }

    /// Get height at given abcissa
    fn get_height(&self, x: i32) -> i32 {
        let mut heights = vec![0];
        for (previous_h, next_h) in zip(&self.land, self.land[1..].iter()) {
            if x == previous_h.0 {
                heights.push(previous_h.1);
            }
            if x == next_h.0 {
                heights.push(next_h.1);
            }
            if previous_h.0 < x && x < next_h.0 {
                heights.push(
                    previous_h.1
                        + (x - previous_h.0) * (previous_h.1 - next_h.1)
                            / (previous_h.0 - next_h.0),
                );
            }
        }
        // SAFETY: heights contains at least a 0
        *heights.iter().max().unwrap()
    }
}

#[derive(Debug)]
struct Lander {
    height: i32,
    h_speed: i32, // horizontal speed
    v_speed: i32, // vertical speed
    x: i32,       //horizontal position
    rotation: i32,
    thrust: i32,
    target: (i32, i32),
    phase: LandingPhase,
}

#[derive(Debug, Clone, Copy)]
enum Direction {
    Left,
    Right,
}

#[derive(Debug, Clone, Copy)]
enum LandingPhase {
    Direct,
    Horizontal(Direction),
    Landing,
}

impl Lander {
    fn new() -> Self {
        Self {
            height: 0,
            h_speed: 0,
            v_speed: 0,
            x: 0,
            rotation: 0,
            thrust: 0,
            target: (0, 0),
            phase: LandingPhase::Direct,
        }
    }

    fn set_landing_phase(&mut self) {
        if self.h_speed.abs() < 20
            && self.v_speed.abs() < 20
            && self.height - self.target.1 < 200
            && (self.x - self.target.0).abs() < 1000
        {
            self.phase = LandingPhase::Landing;
            return;
        }
        let next_phase = get_landing_phase((self.x, self.height), self.target);
        match self.phase {
            LandingPhase::Direct => self.phase = next_phase,
            LandingPhase::Horizontal(_) => match next_phase {
                LandingPhase::Direct | LandingPhase::Landing => self.phase = next_phase,
                LandingPhase::Horizontal(_) => {
                    // continue in the same direction
                }
            },
            LandingPhase::Landing => {}
        }
    }

    fn set_horizontal_translation(&mut self, dir: Direction) {
        self.set_rotation();
        if self.h_speed.abs() > MAX_SPEED {
            self.rotation = 0;
        }
        if self.v_speed < -1 {
            self.rotation = 0;
            self.thrust = 4;
        } else if self.v_speed > 1 {
            if self.rotation.abs() < MAX_ROTATION {
                self.thrust = 0;
            }
        } else {
            if self.rotation.abs() > MAX_ROTATION {
                self.rotation = match dir {
                    Direction::Left => -MAX_ROTATION,
                    Direction::Right => MAX_ROTATION,
                }
            }
        }
    }

    fn set_rotation(&mut self) {
        self.rotation = get_rotation(self.h_speed, self.x - self.target.0);
    }

    fn set_thrust(&mut self) {
        let kp = 1;
        let kv = 4;
        let sp = kp * (self.height - self.target.1) + kv * self.v_speed;
        eprintln!(
            "Debug message... Thrust {} = {} x {} + {} x {}",
            sp,
            kp,
            self.height - self.target.1,
            kv,
            self.v_speed
        );
        self.thrust = sp.min(4).max(0);
    }

    fn set_rotation_thrust(&mut self) {
        match &self.phase {
            LandingPhase::Direct => {
                self.set_thrust();
                self.set_rotation();
            }
            LandingPhase::Horizontal(dir) => {
                self.set_horizontal_translation(*dir);
            }
            LandingPhase::Landing => {
                self.thrust = 0;
                self.rotation = 0;
            }
        }
    }

    fn set_limits(&mut self) {
        let h_speed_limit = 50;
        if self.h_speed.abs() > h_speed_limit {
            println!("Debug message... speed limit");
            self.rotation = 0;
        }
        if self.rotation.abs() > 30 {
            println!("Debug message... excesive rotation");
            self.thrust = 4;
        }
    }
}

fn main() {
    let mut land: Vec<(i32, i32)> = Vec::new();
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let surface_n = parse_input!(input_line, i32); // the number of points used to draw the surface of Mars.
    let mut lander: Lander = Lander::new();
    for i in 0..surface_n as usize {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let inputs = input_line.split(' ').collect::<Vec<_>>();
        let land_x = parse_input!(inputs[0], i32); // X coordinate of a surface point. (0 to 6999)
        let land_y = parse_input!(inputs[1], i32); // Y coordinate of a surface point. By linking all the points together in a sequential fashion, you form the surface of Mars.
        land.push((land_x, land_y));
        //
    }
    let land = Terrain { land };
    let landing_site = land.get_landing_site();
    lander.target = landing_site;
    dbg!("{:?}", &landing_site);
    dbg!("{:?}", &land);

    // game loop
    loop {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let inputs = input_line.split(' ').collect::<Vec<_>>();
        let x = parse_input!(inputs[0], i32);
        let y = parse_input!(inputs[1], i32);
        let h_speed = parse_input!(inputs[2], i32); // the horizontal speed (in m/s), can be negative.
        let v_speed = parse_input!(inputs[3], i32); // the vertical speed (in m/s), can be negative.
        let fuel = parse_input!(inputs[4], i32); // the quantity of remaining fuel in liters.
        let rotate = parse_input!(inputs[5], i32); // the rotation angle in degrees (-90 to 90).
        let power = parse_input!(inputs[6], i32); // the thrust power (0 to 4).

        eprintln!("Debug message... lander {:?}", lander);
        lander.x = x;
        lander.height = y;
        lander.h_speed = h_speed;
        lander.v_speed = v_speed;

        lander.set_landing_phase();
        lander.set_rotation_thrust();
        eprintln!("Debug message... lander {:?}", lander);
        println!("{} {}", lander.rotation, lander.thrust);
    }
}
