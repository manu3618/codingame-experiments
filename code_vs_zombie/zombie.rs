use itertools::iproduct;
use std::io;

macro_rules! parse_input {
    ($x:expr, $t:ident) => {
        $x.trim().parse::<$t>().unwrap()
    };
}

#[derive(Debug, Clone)]
struct Zombie {
    id: i32,
    x: i32,
    y: i32,
    x_next: i32,
    y_next: i32,
}

#[derive(Debug, Clone)]
struct Human {
    id: i32,
    x: i32,
    y: i32,
}

/**
 * Save humans, destroy zombies!
 **/
fn main() {
    // game loop
    loop {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let inputs = input_line.split(" ").collect::<Vec<_>>();
        let x = parse_input!(inputs[0], i32);
        let y = parse_input!(inputs[1], i32);
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let human_count = parse_input!(input_line, i32);
        let mut humans: Vec<Human> = Vec::new();
        for i in 0..human_count as usize {
            let mut input_line = String::new();
            io::stdin().read_line(&mut input_line).unwrap();
            let inputs = input_line.split(" ").collect::<Vec<_>>();
            let human_id = parse_input!(inputs[0], i32);
            let human_x = parse_input!(inputs[1], i32);
            let human_y = parse_input!(inputs[2], i32);
            let hum = Human {
                id: human_id,
                x: human_x,
                y: human_y,
            };
            humans.push(hum)
        }
        eprintln!("Debug message... humans {:?}", humans);

        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let zombie_count = parse_input!(input_line, i32);
        let mut zombies: Vec<Zombie> = Vec::new();
        for i in 0..zombie_count as usize {
            let mut input_line = String::new();
            io::stdin().read_line(&mut input_line).unwrap();
            let inputs = input_line.split(" ").collect::<Vec<_>>();
            let zombie_id = parse_input!(inputs[0], i32);
            let zombie_x = parse_input!(inputs[1], i32);
            let zombie_y = parse_input!(inputs[2], i32);
            let zombie_xnext = parse_input!(inputs[3], i32);
            let zombie_ynext = parse_input!(inputs[4], i32);

            let zomb = Zombie {
                id: zombie_id,
                x: zombie_x,
                y: zombie_y,
                x_next: zombie_xnext,
                y_next: zombie_ynext,
            };
            zombies.push(zomb)
        }

        eprintln!("Debug message... zombies {:?}", zombies);
        // Write an action using println!("message...");
        // To debug: eprintln!("Debug message...");

        // opportunity kill
        let nearest_zomb = get_nearest_zombie(zombies.clone(), x, y);
        if (nearest_zomb.x_next - x).pow(2) + (nearest_zomb.y_next - y).pow(2) < 2000_i32.pow(2) {
            println!("{} {}", nearest_zomb.x_next, nearest_zomb.y_next);
            continue;
        }

        let (z, h) = get_most_threatening_zombie(zombies, humans);
        if ((h.x - x).pow(2) + (h.y - y).pow(2)) < ((z.x - x).pow(2) + (z.y - y).pow(2)) {
            // go to human
            println!("{} {}", h.x, h.y);
        } else {
            // go to zombie
            println!("{} {}", z.x_next, z.y_next);
        }
    }
}

fn get_nearest_zombie(zombies: Vec<Zombie>, x: i32, y: i32) -> Zombie {
    let mut zomb: Zombie = zombies[0].clone();
    let mut min_square_dist = (zomb.x - x).pow(2) + (zomb.y - y).pow(2);
    for z in zombies.iter() {
        let sq = (z.x - x).pow(2) + (z.y - y).pow(2);
        if sq < min_square_dist {
            zomb = z.clone();
            min_square_dist = sq;
        }
    }
    zomb
}

/// Given distance between humans and zombies, get the zombie that is most
/// likely to kill human early
fn get_most_threatening_zombie(zombies: Vec<Zombie>, humans: Vec<Human>) -> (Zombie, Human) {
    let mut zomb: Zombie = zombies[0].clone();
    let mut huma: Human = humans[0].clone();
    let mut min_square_dist = (zomb.x - huma.x).pow(2) + (zomb.y - huma.y).pow(2);

    for (z, h) in iproduct!(zombies.iter(), humans.iter()) {
        let sq = (z.x - h.x).pow(2) + (z.y - h.y).pow(2);
        if sq < min_square_dist {
            zomb = z.clone();
            huma = h.clone();
            min_square_dist = sq;
        }
    }
    (zomb, huma)
}
