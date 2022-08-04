// https://www.codingame.com/ide/puzzle/line-racing
use itertools::iproduct;
use std::collections::HashMap;
use std::collections::HashSet;
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
    // game loop
    let mut players: Vec<(usize, usize)> = Vec::new(); // coordinates of all players
    players.resize(2, (0, 0)); // max number of players
    let mut next_step: (usize, usize) = (0, 0);

    // playgroung
    // '.' : free space
    // '<n>' : trace let by player n
    let mut ground = vec![vec!['.'; 30_usize]; 20_usize];
    let mut previous_dir = "UP";
    let mut fill = false;

    loop {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let inputs = input_line.split(" ").collect::<Vec<_>>();
        let n = parse_input!(inputs[0], usize); // total number of players (2 to 4).
        let p = parse_input!(inputs[1], usize); // your player number (0 to 3).
                                                //
        for i in 0..n {
            let mut input_line = String::new();
            io::stdin().read_line(&mut input_line).unwrap();
            let inputs = input_line.split(" ").collect::<Vec<_>>();
            let x0 = parse_input!(inputs[0], i32); // starting X coordinate of lightcycle (or -1)
            let y0 = parse_input!(inputs[1], i32); // starting Y coordinate of lightcycle (or -1)
            let x1 = parse_input!(inputs[2], i32); // starting X coordinate of lightcycle (can be the same as X0 if you play before this player)
            let y1 = parse_input!(inputs[3], i32); // starting Y coordinate of lightcycle (can be the same as Y0 if you play before this player)
            eprintln!(
                "Debug message... player {} \t({}, {}) \t({}, {})",
                i, x0, y0, x1, y1
            );

            if x0 == -1 {
                remove_player(n, &mut ground);
            } else {
                players[i] = (y1 as usize, x1 as usize); // row (y) , col (x)
                ground[y1 as usize][x1 as usize] = i.to_string().chars().next().unwrap();
                ground[y0 as usize][x0 as usize] = i.to_string().chars().next().unwrap();
            }
        }

        // Write an action using println!("message...");
        // To debug: eprintln!("Debug message...");

        // next_step = match get_neighbours_fill(players[p], &ground, "NE".to_string()).get(0) {
        //     None => (0_usize, 0_usize),
        //     Some(x) => *x,
        // };

        display_ground(&ground);

        if fill {
            next_step = get_nextstep_fill_rotate(players[p], &previous_dir, &ground, true);
        } else {
            let mut goal: (usize, usize) = (0, 0);
            let mut goal_char = '0';
            if p != 0 {
                goal_char = '1';
            }
            goal = get_coordinates(goal_char, &ground).unwrap();
            next_step = get_path(players[p], goal, &ground);
            if players[p].0 <= 0 || players[p].0 >= 19 || players[p].1 <= 0 || players[p].1 >= 29 {
                fill = true;
            }
        }
        eprintln!(
            "Debug message... {:?} \t -> {:?} ({})",
            players[p], next_step, fill
        );

        if next_step.0 < players[p].0 {
            previous_dir = "UP";
            println!("UP");
        }
        if next_step.0 > players[p].0 {
            previous_dir = "DOWN";
            println!("DOWN");
        }
        if next_step.1 < players[p].1 {
            previous_dir = "LEFT";
            println!("LEFT");
        }
        if next_step.1 > players[p].1 {
            previous_dir = "RIGHT";
            println!("RIGHT")
        }
    }
}

fn remove_player(n: usize, ground: &mut Vec<Vec<char>>) {
    let height = ground.len();
    let width = ground[0].len();
    let to_delete = n.to_string().chars().next().unwrap();
    for (row, col) in iproduct!(0..height, 0..width) {
        if ground[row][col] == to_delete {
            ground[row][col] = '.';
        }
    }
}

fn get_neighbours(p: (usize, usize), ground: &Vec<Vec<char>>) -> Vec<(usize, usize)> {
    let height = ground.len();
    let width = ground[0].len();
    let mut neighbours = Vec::new();
    let coords = vec![
        (p.0, p.1 - 1),
        (p.0, p.1 + 1),
        (p.0 + 1, p.1),
        (p.0 - 1, p.1),
    ];

    for (row, col) in coords {
        if row < 0 || row >= height {
            continue;
        }
        if col < 0 || col >= width {
            continue;
        }
        if ground[row][col] == '.' {
            neighbours.push((row, col));
        }
    }
    neighbours
}

/// Get neighbour to floodfill ground
/// snake based algorythm
fn get_neighbours_fill(
    p: (usize, usize),
    ground: &Vec<Vec<char>>,
    direction: String,
) -> Vec<(usize, usize)> {
    let height = ground.len();
    let width = ground[0].len();
    let mut neighbours = Vec::new();
    let mut coords: Vec<(usize, usize)> = Vec::new();

    for dir in direction.chars() {
        match dir {
            'N' => {
                coords.push((p.0 + 1, p.1));
                coords.push((p.0 - 1, p.1));
            }
            'S' => {
                coords.push((p.0 + 1, p.1));
                coords.push((p.0 - 1, p.1));
            }
            'E' => {
                coords.push((p.0, p.1 + 1));
                coords.push((p.0, p.1 - 1));
            }
            'W' => {
                coords.push((p.0, p.1 - 1));
                coords.push((p.0, p.1 + 1));
            }
            _ => {}
        }
    }
    coords.push((p.0 + 1, p.1));
    coords.push((p.0 - 1, p.1));
    coords.push((p.0, p.1 + 1));
    coords.push((p.0, p.1 - 1));

    for (row, col) in coords {
        if row < 0 || row >= height {
            continue;
        }
        if col < 0 || col >= width {
            continue;
        }
        if ground[row][col] == '.' {
            neighbours.push((row, col));
        }
    }
    neighbours
}

/// Get next step to fill a volume following walls
fn get_nextstep_fill_rotate(
    p: (usize, usize),
    current_dir: &str,
    ground: &Vec<Vec<char>>,
    dextrogyre: bool,
) -> (usize, usize) {
    let height = ground.len();
    let width = ground[0].len();
    let mut coords: Vec<(usize, usize)> = Vec::new();

    match (current_dir, dextrogyre) {
        ("LEFT", true) => {
            coords = vec![(p.0 + 1, p.1), (p.0, p.1 - 1), (p.0 - 1, p.1)];
        }
        ("LEFT", false) => {
            coords = vec![(p.0 - 1, p.1), (p.0, p.1 - 1), (p.0 + 1, p.1)];
        }
        ("UP", true) => {
            coords = vec![(p.0, p.1 - 1), (p.0 - 1, p.1), (p.0, p.1 + 1)];
        }
        ("UP", false) => {
            coords = vec![(p.0, p.1 + 1), (p.0 - 1, p.1), (p.0, p.1 - 1)];
        }
        ("RIGHT", true) => {
            coords = vec![(p.0 - 1, p.1), (p.0, p.1 + 1), (p.0 + 1, p.1)];
        }
        ("RIGHT", false) => {
            coords = vec![(p.0 + 1, p.1), (p.0, p.1 + 1), (p.0 - 1, p.1)];
        }
        ("DOWN", true) => {
            coords = vec![(p.0, p.1 + 1), (p.0 + 1, p.1), (p.0, p.1 - 1)];
        }
        ("DOWN", false) => {
            coords = vec![(p.0, p.1 + 1), (p.0 + 1, p.1), (p.0, p.1 - 1)];
        }
        _ => {
            coords = vec![
                (p.0 + 1, p.1),
                (p.0 - 1, p.1),
                (p.0, p.1 + 1),
                (p.0, p.1 - 1),
            ];
        }
    }
    let first = (coords[0].0, coords[0].1);

    for (row, col) in coords {
        if row < 0 || row >= height {
            continue;
        }
        if col < 0 || col >= width {
            continue;
        }
        if ground[row][col] == '.' {
            return (row, col);
        }
    }
    return first;
}

/// Quick and dirty A*
/// Return next hop
fn get_path(
    start: (usize, usize),
    goal: (usize, usize),
    ground: &Vec<Vec<char>>,
) -> (usize, usize) {
    // let now = time::Instant::now();
    let mut visited = HashMap::new(); // 1st_hop : HeshSet{(uize, usize)}
    let max_cost = 100;
    let candidate_hop = get_neighbours(start, ground);
    if candidate_hop.len() == 1 {
        return candidate_hop[0];
    }
    if candidate_hop.contains(&goal) {
        return goal;
    }
    for hop in candidate_hop.iter() {
        let mut values = HashSet::new();
        values.insert(*hop);
        visited.insert(hop, values);
    }

    // explore one more level of neighbors
    for cost in 0..max_cost {
        for (&candidate, reached) in visited.iter_mut() {
            let mut next_neighbours: Vec<(usize, usize)> = Vec::new();
            let reached_copy = reached.clone();
            for r in reached.iter() {
                for n in get_neighbours(*r, ground).iter() {
                    if *n == goal {
                        return *candidate;
                    }
                    if !reached_copy.contains(n) {
                        next_neighbours.push((n.0, n.1));
                    }
                }
            }
            if next_neighbours.len() > 0 {
                reached.extend(next_neighbours.as_slice());
            }
        }
    }
    return candidate_hop[0];
}

fn get_coordinates(c: char, ground: &Vec<Vec<char>>) -> Option<(usize, usize)> {
    let height = ground.len();
    let width = ground[0].len();
    let row_range = 0..height;
    let col_range = 0..width;

    for (row, col) in iproduct!(row_range, col_range) {
        if ground[row][col] == c {
            return Some((row, col));
        }
    }
    return None;
}

fn display_ground(ground: &Vec<Vec<char>>) {
    for row in ground {
        eprintln!("Debug Message... {}", row.into_iter().collect::<String>());
    }
}
