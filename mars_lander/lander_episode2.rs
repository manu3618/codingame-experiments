use std::cmp;
use std::io;

macro_rules! parse_input {
    ($x:expr, $t:ident) => {
        $x.trim().parse::<$t>().unwrap()
    };
}

/**
 * Auto-generated code below aims at helping you parse
 * the standard input according to the problem statement.
 **/
fn main() {
    let mut land: Vec<(i32, i32)> = Vec::new();
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let surface_n = parse_input!(input_line, i32); // the number of points used to draw the surface of Mars.
    let mut set_rotation = 0;
    let mut set_power = 0;
    for i in 0..surface_n as usize {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let inputs = input_line.split(" ").collect::<Vec<_>>();
        let land_x = parse_input!(inputs[0], i32); // X coordinate of a surface point. (0 to 6999)
        let land_y = parse_input!(inputs[1], i32); // Y coordinate of a surface point. By linking all the points together in a sequential fashion, you form the surface of Mars.
        land.push((land_x, land_y));
        //
    }

    let landing_site = get_landing_site(&land);

    // game loop
    loop {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let inputs = input_line.split(" ").collect::<Vec<_>>();
        let x = parse_input!(inputs[0], i32);
        let y = parse_input!(inputs[1], i32);
        let h_speed = parse_input!(inputs[2], i32); // the horizontal speed (in m/s), can be negative.
        let v_speed = parse_input!(inputs[3], i32); // the vertical speed (in m/s), can be negative.
        let fuel = parse_input!(inputs[4], i32); // the quantity of remaining fuel in liters.
        let rotate = parse_input!(inputs[5], i32); // the rotation angle in degrees (-90 to 90).
        let power = parse_input!(inputs[6], i32); // the thrust power (0 to 4).

        // Write an action using println!("message...");
        // To debug: eprintln!("Debug message...");

        // 2 integers: rotate power. rotate is the desired rotation angle (should be 0 for level 1), power is the desired thrust power (0 to 4).
        let remaining_h = y - get_height(x, &land) as i32;
        eprintln!("Debug message... height {}", remaining_h);
        let margin = z_margin(v_speed, remaining_h);
        eprintln!("Debug message... margin {}", margin);

        if margin.abs() > 20.0 {
            set_power = 0
        } else if margin.abs() > 15.0 {
            set_power = 1
        } else if margin.abs() > 10.0 {
            set_power = 2
        } else if margin.abs() > 5.0 {
            set_power = 3
        } else {
            set_power = 4
        }

        eprintln!("Debug message... h distance {}", landing_site.0 - x);
        let h_margin = x_margin(h_speed, landing_site.0 - x);
        eprintln!("Debug message... h margin {}", h_margin);
        set_rotation = get_rotation(h_speed, landing_site.0 - x);

        if h_speed.abs() < 20
            && (landing_site.0 - x).abs() < 500
            && remaining_h < 10
            && v_speed.abs() < 30
        {
            // landing conditions
            eprintln!("Debug message... landing");
            set_rotation = 0;
        }

        if set_rotation.abs() > 5 {
            set_power = 4;
        }

        println!("{} {}", set_rotation, set_power);
    }
}

/// compute margin
/// if absolute bvalue of margin is greater than 1, then we can let the lander
/// freefall
/// if the absolute value of margin is lower than one, the full thrust is required
/// to avoir crash
fn z_margin(h_speed: i32, height: i32) -> f64 {
    let max_thrust = 4.0;
    let max_a = max_thrust - 3.711;
    ((height as f64) * max_a) / (2.0 * h_speed as f64)
}

fn x_margin(x_speed: i32, dist: i32) -> f64 {
    let max_a = 1.0;
    ((dist as f64) * max_a) / (2.0 * x_speed as f64)
}

/// find terrain height at specific x
fn get_height(x: i32, land: &Vec<(i32, i32)>) -> f64 {
    let mut previous_h = (0.0, 0.0);
    let mut next_h = (0.0, 0.0);
    for point in land {
        if point.0 == x {
            return point.1.into();
        } else if point.0 < x {
            previous_h = (point.0 as f64, point.1 as f64);
        } else {
            next_h = (point.0 as f64, point.1 as f64);
            break;
        }
    }
    previous_h.1 + (x as f64 - previous_h.0) * (previous_h.1 - next_h.1) / (previous_h.0 - next_h.0)
}

/// compute middle of langding site coordinates
fn get_landing_site(land: &Vec<(i32, i32)>) -> (i32, i32) {
    let mut previous = (0, 0);
    for point in land {
        if point.1 == previous.1 {
            return ((previous.0 + point.0) / 2, previous.1);
        }

        previous = (point.0, point.1);
    }
    previous
}

fn get_rotation(h_speed: i32, landing_distance: i32) -> i32 {
    let k0 = 2;
    let k1 = 1;
    let result = -k0 * landing_distance / 100 + k1 * h_speed;
    eprintln!(
        "Debug message... speed {} distance {} result {}",
        h_speed, landing_distance, result
    );
    return cmp::max(-90, cmp::min(result, 90));
}
