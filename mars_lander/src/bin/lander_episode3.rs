// https://www.codingame.com/training/expert/mars-lander-episode-3
use std::f64::consts::PI;
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
/// if absolute value of margin is greater than 1, then we can let the lander
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
    let result = k0 * landing_distance / 100 + k1 * h_speed;
    let max_rotation = 45;
    eprintln!(
        "Debug message... speed {} distance {} result {}",
        h_speed, landing_distance, result
    );
    result.min(max_rotation).max(-max_rotation)
}

fn get_extremity(dir: Direction, p: (i32, i32)) -> (i32, i32) {
    match dir {
        Direction::Right => (6999, p.1),
        Direction::Left => (0, p.1),
    }
}

/// Get possible landing phase given the landoer position and the landig site
fn get_landing_phase(position: (i32, i32), target: (i32, i32), terrain: &Terrain) -> LandingPhase {
    let line = Line {
        p0: position,
        p1: target,
    };
    let mut landing_phase = LandingPhase::Direct;
    if let Some((a, _)) = line.get_equation() {
        eprintln!("Debug message... landign phase a: {:?}", a);
        if a.abs() < 0.25 || terrain.has_conflict(line) {
            if position.0 < target.0 {
                landing_phase = LandingPhase::Horizontal(Direction::Right);
            } else {
                landing_phase = LandingPhase::Horizontal(Direction::Left);
            }
        }
    }
    if let LandingPhase::Horizontal(dir) = landing_phase {
        let line = Line {
            p0: get_extremity(dir, position),
            p1: position,
        };
        if terrain.has_conflict(line) {
            landing_phase = LandingPhase::Up(dir);
        }
    }
    landing_phase
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
        if self.p0.0 == self.p1.0 {
            return None;
        }
        let a = (self.p0.1 - self.p1.1) as f64 / (self.p0.0 - self.p1.0) as f64;
        let b0 = self.p0.1 as f64 - a * self.p0.0 as f64;
        let b1 = self.p1.1 as f64 - a * self.p1.0 as f64;
        assert!((b0 - b1).abs() < 1.0);
        Some((a, b0))
    }

    /// get intersection point between 2 lines. a line is defined by 2 points
    fn get_intersection(&self, other: &Line) -> Option<(f64, f64)> {
        if let (Some((a, b)), Some((c, d))) = (self.get_equation(), other.get_equation()) {
            let x = (d - b) / (a - c);
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
    fn has_conflict(&self, direct: Line) -> bool {
        for (previous, next) in zip(&self.land, self.land[1..].iter()) {
            let segment = Line {
                p0: *previous,
                p1: *next,
            };
            if segment.p0.1 == segment.p1.1 {
                //landing site
                continue;
            }
            if let Some((x, y)) = direct.get_intersection(&segment) {
                if x > f64::min(segment.p0.0 as f64, segment.p1.0 as f64)
                    && x < f64::max(segment.p0.0 as f64, segment.p1.0 as f64)
                {
                    eprintln!("Debug message... collision detected");
                    eprintln!("Debug message... collision trajectory {:?}", direct);
                    eprintln!("Debug message... equation {:?}", direct.get_equation());
                    eprintln!("Debug message... collision land {:?}", &segment);
                    eprintln!("Debug message... equation {:?}", &segment.get_equation());
                    eprintln!("Debug message... coords {:?}", (x, y));
                    return true;
                }
            }
        }
        false
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
    Up(Direction), // must go up to pass over a hump
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

    fn set_landing_phase(&mut self, terrain: &Terrain) {
        if self.h_speed.abs() < 20
            && self.v_speed.abs() < 20
            && self.height - self.target.1 < 200
            && (self.x - self.target.0).abs() < 1000
        {
            self.phase = LandingPhase::Landing;
            eprintln!("Debug message... landing phase {:?}", self.phase);
            return;
        }
        let next_phase = get_landing_phase((self.x, self.height), self.target, terrain);
        match self.phase {
            LandingPhase::Direct | LandingPhase::Up(_) => self.phase = next_phase,
            LandingPhase::Horizontal(_) => match next_phase {
                LandingPhase::Direct | LandingPhase::Landing | LandingPhase::Up(_) => {
                    self.phase = next_phase
                }
                LandingPhase::Horizontal(_) => {
                    // continue in the same direction
                }
            },
            LandingPhase::Landing => {
                // we may have overrun the landing site
                self.phase = next_phase
            }
        }
        eprintln!("Debug message... landing phase {:?}", self.phase);
    }

    /// Adjust rotation and thrust to obtain
    ///
    /// * zero vertical velocity
    /// * MAX_SPEED horizontal velocity
    fn set_horizontal_translation(&mut self, dir: Direction) {
        if self.h_speed.abs() >= MAX_SPEED {
            self.rotation = match dir {
                Direction::Left => -MAX_ROTATION,
                Direction::Right => MAX_ROTATION,
            }
        } else {
            self.rotation = match dir {
                Direction::Left => MAX_ROTATION,
                Direction::Right => -MAX_ROTATION,
            }
        }
        if self.v_speed < -1 {
            self.rotation = 0;
            self.thrust = 4;
        } else if self.v_speed > 1 {
            self.thrust = 3;
        } else if self.rotation.abs() > MAX_ROTATION {
            self.rotation = match dir {
                Direction::Left => MAX_ROTATION,
                Direction::Right => -MAX_ROTATION,
            }
        }
    }

    fn set_rotation(&mut self) {
        self.rotation = get_rotation(self.h_speed, self.x - self.target.0);
    }

    fn set_thrust(&mut self) {
        let margin = z_margin(self.v_speed, self.height - self.target.1);
        eprintln!("Debug message... z_margin {}", &margin);
        let kp = -5;
        let kv = -10;
        let sp = kp * (self.height - self.target.1) / 100 + kv * self.v_speed;
        eprintln!(
            "Debug message... Thrust {} = {} x {} + {} x {}",
            sp,
            kp,
            self.height - self.target.1,
            kv,
            self.v_speed
        );
        let sp = (sp as f64 / (PI / 180.0).cos()) as i32;
        self.thrust = sp.min(4).max(0);

        if margin.abs() < 1.1 {
            self.thrust = 4;
        } else {
            self.thrust = -self.v_speed / 5;
        }
    }

    /// Set both rotation and thrust depending on landing phase and target
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
            LandingPhase::Up(dir) => {
                self.set_horizontal_translation(*dir);
                self.thrust = 4;
                self.rotation = self.rotation / 3;
            }
        }
        self.set_limits();
    }

    /// put parameters closer to central limits so that parameters stay in control
    fn set_limits(&mut self) {
        if self.h_speed.abs() > MAX_SPEED {
            eprintln!("Debug message SHOULD NOT BE REACHED... speed limit");
            if self.h_speed > 0 {
                self.rotation = self.rotation.abs();
            } else {
                self.rotation = -self.rotation.abs();
            }
        }
        if self.rotation.abs() > 30 {
            eprintln!("Debug message SHOULD NOT BE REACHED... excesive rotation");
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
    for _i in 0..surface_n as usize {
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
        eprintln!(
            "Debug message... inputs {:?}, {:?}; {:?}",
            fuel, rotate, power
        );

        eprintln!("Debug message... lander {:?}", lander);
        lander.x = x;
        lander.height = y;
        lander.h_speed = h_speed;
        lander.v_speed = v_speed;

        lander.set_landing_phase(&land);
        lander.set_rotation_thrust();
        eprintln!("Debug message... lander {:?}", lander);
        println!("{} {}", lander.rotation, lander.thrust);
    }
}
