// https://www.codingame.com/ide/puzzle/line-racing
use itertools::iproduct;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fmt;
use std::fmt::Display;
use std::io;

// TODO: computer longest path (algorithm as in TAN)

macro_rules! parse_input {
    ($x:expr, $t:ident) => {
        $x.trim().parse::<$t>().unwrap()
    };
}

#[derive(Debug)]
struct Playground {
    ground: Vec<Vec<char>>,
}

impl Playground {
    fn new() -> Self {
        Self {
            ground: vec![vec!['.'; 30_usize]; 20_usize],
        }
    }

    fn remove_player(&mut self, player_num: usize) {
        let height = self.ground.len();
        let width = self.ground[0].len();
        let to_delete = player_num.to_string().chars().next().unwrap();
        for (row, col) in iproduct!(0..height, 0..width) {
            if self.ground[row][col] == to_delete {
                self.ground[row][col] = '.';
            }
        }
    }

    fn get_empty_neighbors(&self, point: (usize, usize)) -> Vec<(usize, usize)> {
        let height = self.ground.len();
        let width = self.ground[0].len();
        let mut neighbors = Vec::new();
        if point.0 > 0 && point.1 > 0 && self.ground[point.0 - 1][point.1 - 1] == '.' {
            neighbors.push((point.0 - 1, point.1 - 1));
        }
        if point.0 > 0 && point.1 < width && self.ground[point.0 - 1][point.1 + 1] == '.' {
            neighbors.push((point.0 - 1, point.1 + 1));
        }
        if point.0 < height && point.1 > 0 && self.ground[point.0 + 1][point.1 - 1] == '.' {
            neighbors.push((point.0 + 1, point.1 - 1));
        }
        if point.0 < height && point.1 < width && self.ground[point.0 + 1][point.1 + 1] == '.' {
            neighbors.push((point.0 + 1, point.1 + 1));
        }
        neighbors
    }

    /// Get longest path for all reachable destination
    fn longest_paths(
        &self,
        start_point: (usize, usize),
    ) -> HashMap<(usize, usize), Vec<(usize, usize)>> {
        let mut longests = HashMap::new();
        longests.insert(start_point, Vec::new());

        let mut new_paths: Vec<Vec<(usize, usize)>> = Vec::new();
        let mut modified = true; // has longests been modified
        while modified {
            modified = false;
            for path in longests.values() {
                let neighbors =
                    self.get_empty_neighbors(*path.last().expect("contains at least start point"));
                for neighbor in neighbors {
                    if path.contains(&neighbor) {
                        continue;
                    }
                    let mut new_path = path.clone();
                    new_path.push(neighbor);
                    new_paths.push(new_path);
                }
            }
            for path in &new_paths {
                let dst = path.last().expect("contains at least start point");
                if let Some(current_path) = longests.get_mut(dst) {
                    if current_path.len() < path.len() {
                        *current_path = path.clone();
                    }
                } else {
                    longests.insert(*dst, path.clone());
                    modified = true;
                }
            }
        }
        longests
    }
    fn longest_path(&self, start_point: (usize, usize)) -> Vec<(usize, usize)> {
        let destinations = self.longest_paths(start_point);
        let mut paths: Vec<&Vec<(usize, usize)>> = destinations.values().collect();
        if paths.len() == 0 {
            return Vec::new();
        }
        paths.sort_by_key(|x| x.len());
        (*paths.last().expect("early return if empty")).to_vec()
    }

    /// get next step in the longest path found
    fn next_step(&self, start_point: (usize, usize)) -> Option<(usize, usize)> {
        if let Some(x) = self.longest_path(start_point).get(0) {
            Some(*x)
        } else {
            None
        }
    }
    /// get next direction to follow the longest path found
    fn next_dir_longest(&self, start_point: (usize, usize), cur_dir: String) -> String {
        if let Some(step) = self.next_step(start_point) {
            if step.0 < start_point.0 {
                return "UP".into();
            } else if step.0 > start_point.0 {
                return "DOWN".into();
            } else if step.1 < start_point.1 {
                return "LEFT".into();
            } else if step.1 > start_point.1 {
                return "RIGHT".into();
            } else {
                unreachable!();
            }
        } else {
            cur_dir
        }
    }
}

impl Display for Playground {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut lines: Vec<String> = Vec::new();

        for row in &self.ground {
            lines.push(row.into_iter().collect::<String>());
        }
        Display::fmt(&lines.join("\n").to_string(), f)
    }
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
    let mut ground = Playground::new();
    let mut previous_dir = String::from("UP");
    let mut fill = false;

    loop {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let inputs = input_line.split(' ').collect::<Vec<_>>();
        let n = parse_input!(inputs[0], usize); // total number of players (2 to 4).
        let p = parse_input!(inputs[1], usize); // your player number (0 to 3).

        for i in 0..n {
            let mut input_line = String::new();
            io::stdin().read_line(&mut input_line).unwrap();
            let inputs = input_line.split(' ').collect::<Vec<_>>();
            let x0 = parse_input!(inputs[0], i32); // starting X coordinate of lightcycle (or -1)
            let y0 = parse_input!(inputs[1], i32); // starting Y coordinate of lightcycle (or -1)
            let x1 = parse_input!(inputs[2], i32); // starting X coordinate of lightcycle (can be the same as X0 if you play before this player)
            let y1 = parse_input!(inputs[3], i32); // starting Y coordinate of lightcycle (can be the same as Y0 if you play before this player)
            eprintln!(
                "Debug message... player {} \t({}, {}) \t({}, {})",
                i, x0, y0, x1, y1
            );

            if x0 == -1 {
                ground.remove_player(n);
            } else {
                players[i] = (y1 as usize, x1 as usize); // row (y) , col (x)
                ground.ground[y1 as usize][x1 as usize] = i.to_string().chars().next().unwrap();
                ground.ground[y0 as usize][x0 as usize] = i.to_string().chars().next().unwrap();
            }
        }

        // Write an action using println!("message...");
        // To debug: eprintln!("Debug message...");

        // next_step = match get_neighbours_fill(players[p], &ground, "NE".to_string()).get(0) {
        //     None => (0_usize, 0_usize),
        //     Some(x) => *x,
        // };

        eprintln!("{}", &ground);

        if fill {
            next_step = get_nextstep_fill_rotate(players[p], &previous_dir, &ground.ground, true);
            eprintln!(
                "Debug message... {:?} \t -> {:?} ({})",
                players[p], next_step, fill
            );

            if next_step.0 < players[p].0 {
                previous_dir = "UP".into();
                println!("UP");
            }
            if next_step.0 > players[p].0 {
                previous_dir = "DOWN".into();
                println!("DOWN");
            }
            if next_step.1 < players[p].1 {
                previous_dir = "LEFT".into();
                println!("LEFT");
            }
            if next_step.1 > players[p].1 {
                previous_dir = "RIGHT".into();
                println!("RIGHT")
            }
        } else {
            previous_dir = ground.next_dir_longest(players[p], previous_dir);
            println!("{}", previous_dir);
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
        if row == 0 || row >= height {
            continue;
        }
        if col == 0 || col >= width {
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
        if row == 0 || row >= height {
            continue;
        }
        if col == 0 || col >= width {
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
        if row == 0 || row >= height {
            continue;
        }
        if col == 0 || col >= width {
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
