// https://www.codingame.com/ide/puzzle/power-of-thor-episode-2
use std::io;

macro_rules! parse_input {
    ($x:expr, $t:ident) => {
        $x.trim().parse::<$t>().unwrap()
    };
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
fn barycenter(giants: &Vec<(i32, i32)>) -> Option<(i32, i32)> {
    if let Some(giant) = giants.iter().copied().reduce(|a, b| (a.0 + b.0, a.1 + b.1)) {
        Some((giant.0 / giants.len() as i32, giant.1 / giants.len() as i32))
    } else {
        None
    }
}

fn get_dir(src: (i32, i32), dst: (i32, i32)) -> String {
    let mut dir = String::from("");
    match dst.1 - src.1 {
        0 => {}
        i32::MIN..=0 => dir.push_str("N"),
        1..=i32::MAX => dir.push_str("S"),
    }
    match dst.0 - src.0 {
        0 => {}
        i32::MIN..=0 => dir.push_str("W"),
        1..=i32::MAX => dir.push_str("E"),
    }

    dir
}

fn main() {
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let inputs = input_line.split(" ").collect::<Vec<_>>();
    let tx = parse_input!(inputs[0], i32); // Thor coordinates
    let ty = parse_input!(inputs[1], i32); // Thor coordinates
    let mut thor = (tx, ty);
    let mut giants: Vec<(i32, i32)> = Vec::new();
    let mut trigger_dist = 2;

    // game loop
    loop {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let inputs = input_line.split(" ").collect::<Vec<_>>();
        let _h = parse_input!(inputs[0], i32); // the remaining number of hammer strikes.
        let n = parse_input!(inputs[1], i32); // the number of giants which are still present on the map.
        giants.truncate(0);
        for _ in 0..n as usize {
            let mut input_line = String::new();
            io::stdin().read_line(&mut input_line).unwrap();
            let inputs = input_line.split(" ").collect::<Vec<_>>();
            let x = parse_input!(inputs[0], i32);
            let y = parse_input!(inputs[1], i32);
            giants.push((x, y));
        }
        let dist = min_dist(&giants, thor);
        if dist <= trigger_dist {
            println!("STRIKE");
            continue;
        }
        if let Some(dst) = barycenter(&giants) {
            let dir = get_dir(thor, dst);
            eprintln!("Debug message {:?} {:?} -> {:?}", dir, &thor, &dst);
            if dir.contains("N") {
                thor.1 -= 1;
            }
            if dir.contains("S") {
                thor.1 += 1;
            }
            if dir.contains("E") {
                thor.0 += 1;
            }
            if dir.contains("W") {
                thor.0 -= 1;
            }
            if dir.len() > 0 {
                println!("{}", dir);
                trigger_dist = 2;
                continue;
            }
        } else {
            println!("WAIT");
            trigger_dist = 1;
            continue;
        }

        eprintln!("Debug message {:?}  ", min_dist(&giants, thor));

        // Write an action using println!("message...");
        // To debug: eprintln!("Debug message...");

        // The movement or action to be carried out: WAIT STRIKE N NE E SE S SW W or N
        println!("WAIT");
    }
}
