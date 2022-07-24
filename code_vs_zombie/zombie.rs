// https://www.codingame.com/ide/puzzle/code-vs-zombies
use itertools::iproduct;
use std::io;

macro_rules! parse_input {
    ($x:expr, $t:ident) => {
        $x.trim().parse::<$t>().unwrap()
    };
}

#[derive(Debug, Clone)]
struct Zombie {
    id: i64,
    x: i64,
    y: i64,
    x_next: i64,
    y_next: i64,
}

#[derive(Debug, Clone)]
struct Human {
    id: i64,
    x: i64,
    y: i64,
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
        let x = parse_input!(inputs[0], i64);
        let y = parse_input!(inputs[1], i64);
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let human_count = parse_input!(input_line, i64);
        let mut humans: Vec<Human> = Vec::new();
        for i in 0..human_count as usize {
            let mut input_line = String::new();
            io::stdin().read_line(&mut input_line).unwrap();
            let inputs = input_line.split(" ").collect::<Vec<_>>();
            let human_id = parse_input!(inputs[0], i64);
            let human_x = parse_input!(inputs[1], i64);
            let human_y = parse_input!(inputs[2], i64);
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
        let zombie_count = parse_input!(input_line, i64);
        let mut zombies: Vec<Zombie> = Vec::new();
        for i in 0..zombie_count as usize {
            let mut input_line = String::new();
            io::stdin().read_line(&mut input_line).unwrap();
            let inputs = input_line.split(" ").collect::<Vec<_>>();
            let zombie_id = parse_input!(inputs[0], i64);
            let zombie_x = parse_input!(inputs[1], i64);
            let zombie_y = parse_input!(inputs[2], i64);
            let zombie_xnext = parse_input!(inputs[3], i64);
            let zombie_ynext = parse_input!(inputs[4], i64);

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

        let (z, h) = get_most_threatening_zombie(zombies, humans, x, y);
        eprintln!("Debug message... go to human {:?}", h.id);
        println!("{} {}", h.x, h.y);
    }
}

fn get_nearest_zombie(zombies: Vec<Zombie>, x: i64, y: i64) -> Zombie {
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

fn get_nearest_human(humans: Vec<Human>, x: i64, y: i64) -> Human {
    let mut huma: Human = humans[0].clone();
    let mut min_square_dist = square_dist(x, y, huma.x, huma.y);
    for h in humans.iter() {
        let sq = square_dist(x, y, h.x, h.y);
        if sq < min_square_dist {
            huma = h.clone();
            min_square_dist = sq;
        }
    }
    huma
}

/// Given distance between humans and zombies, get the zombie that is most
/// likely to kill human early
fn get_most_threatening_zombie(
    zombies: Vec<Zombie>,
    humans: Vec<Human>,
    x: i64,
    y: i64,
) -> (Zombie, Human) {
    let mut zomb = &zombies[0];
    let mut huma = &humans[0];
    let mut max_threat = 0;
    let mut lost_humans: Vec<i64> = Vec::new();

    for (z, h) in iproduct!(zombies.iter(), humans.iter()) {
        if lost_humans.contains(&h.id) {
            continue;
        }
        let threat = get_threat_level(z, h, x, y);
        if threat > 260 {
            lost_humans.push(h.id);
            eprintln!("Debug message... lost humans {:?}", lost_humans);
            continue;
        }
        if max_threat < threat {
            zomb = &z;
            huma = &h;
            max_threat = threat;
        }
    }
    if lost_humans.len() == humans.len() {
        return (zomb.clone(), get_nearest_human(humans, x, y));
    }
    (zomb.clone(), huma.clone())
}

fn square_dist(x1: i64, y1: i64, x2: i64, y2: i64) -> i64 {
    (x1 - x2).pow(2) + (y1 - y2).pow(2)
}

fn get_threat_level(zombie: &Zombie, human: &Human, x: i64, y: i64) -> i64 {
    let zombie_speed = 4_i64.pow(2);
    let my_speed = 20_i64.pow(2);
    let zh = square_dist(human.x, human.y, zombie.x, zombie.y);
    let me_h = square_dist(x, y, human.x, human.y);
    let threat = (me_h * my_speed) / (zh * zombie_speed); //+ human.id; // + zombie.id
    eprintln!(
        "Debug message... z {}\t h{}\t ŧ {}",
        zombie.id, human.id, threat
    );
    threat
}
