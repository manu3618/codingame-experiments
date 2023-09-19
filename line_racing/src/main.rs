// https://www.codingame.com/ide/puzzle/line-racing
// https://www.codingame.com/ide/puzzle/tron-battle
use itertools::iproduct;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fmt;
use std::fmt::Display;
use std::io;

const MAX_LEN: usize = 50; // maximal length for path search
const MAX_DEST: usize = 100; // maximal number of destinations to compute

macro_rules! parse_input {
    ($x:expr, $t:ident) => {
        $x.trim().parse::<$t>().unwrap()
    };
}

#[derive(Debug, Default)]
struct Path(Vec<(usize, usize)>);

#[derive(Debug, Default)]
struct PathCollection(Vec<Vec<(usize, usize)>>);

#[derive(Debug)]
struct Playground {
    ground: Vec<Vec<char>>,
    pathfinding_complete: bool,
    paths: Vec<Vec<(usize, usize)>>,
}

impl Playground {
    fn new() -> Self {
        Self {
            ground: vec![vec!['.'; 30_usize]; 20_usize],
            pathfinding_complete: false,
            paths: Vec::new(),
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
        let height = self.ground.len() - 1;
        let width = self.ground[0].len() - 1;
        let mut neighbors = Vec::new();
        if point.0 > 0 && self.ground[point.0 - 1][point.1] == '.' {
            neighbors.push((point.0 - 1, point.1));
        }
        if point.1 < width && self.ground[point.0][point.1 + 1] == '.' {
            neighbors.push((point.0, point.1 + 1));
        }
        if point.0 < height && self.ground[point.0 + 1][point.1] == '.' {
            neighbors.push((point.0 + 1, point.1));
        }
        if point.1 > 0 && self.ground[point.0][point.1 - 1] == '.' {
            neighbors.push((point.0, point.1 - 1));
        }

        neighbors
    }

    fn longest_dumb_path(&mut self, point: (usize, usize)) -> Vec<Vec<(usize, usize)>> {
        let mut longest = vec![vec![point]];
        let mut modified = true; // has longest been modified
        while modified {
            // dbg!(&longest.len());
            modified = false;
            let mut new_paths: Vec<Vec<(usize, usize)>> = Vec::new();
            for path in &longest {
                let neighbors =
                    self.get_empty_neighbors(*path.last().expect("contains at least start point"));
                for neighbor in neighbors {
                    if path.contains(&neighbor) {
                        continue;
                    }
                    let mut new_path = path.clone();
                    new_path.push(neighbor);
                    if !longest.contains(&new_path) {
                        new_paths.push(new_path);
                        modified = true;
                    }
                }
            }
            longest.extend(new_paths);
            if longest.len() > 3000 {
                break;
            }
        }
        self.paths = longest.clone();
        longest
    }

    fn next_dumb_paths(&mut self, start_point: (usize, usize), cur_dir: String) -> String {
        let tmp = Vec::new();
        self.longest_dumb_path(start_point);
        let longest = self.paths.iter().max_by_key(|x| x.len()).unwrap_or(&tmp);
        // dbg!(self.paths.len());
        // dbg!(longest.len());
        if let Some(step) = longest.get(1) {
            next_dir(start_point, *step).unwrap()
        } else {
            cur_dir
        }
    }

    /// Get longest path for all reachable destination
    fn longest_paths(
        &mut self,
        start_point: (usize, usize),
    ) -> HashMap<(usize, usize), Vec<(usize, usize)>> {
        let mut longests = HashMap::new();
        longests.insert(start_point, vec![start_point]);

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
                // if path.len() > MAX_LEN {
                // if path.len() * longests.len() > MAX_CAP {
                if longests.len() > MAX_DEST || path.len() > MAX_LEN {
                    longests.insert(*dst, path.clone());
                    //dbg!(path.len(), longests.len());
                    self.pathfinding_complete = false;
                    // early retrun
                    self.paths = Vec::new();
                    for path in longests.values() {
                        self.paths.push(path.to_vec());
                    }
                    return longests;
                }
                if let Some(current_path) = longests.get_mut(dst) {
                    if current_path.len() < path.len() {
                        *current_path = path.clone();
                        modified = true;
                    }
                } else {
                    longests.insert(*dst, path.clone());
                    modified = true;
                }
            }
        }
        self.pathfinding_complete = true;
        self.paths = Vec::new();
        for path in longests.values() {
            self.paths.push(path.to_vec());
        }
        longests
    }

    fn longest_path(&mut self, start_point: (usize, usize)) -> Vec<(usize, usize)> {
        let destinations = self.longest_paths(start_point);
        let mut paths: Vec<&Vec<(usize, usize)>> = destinations.values().collect();
        if paths.is_empty() {
            return Vec::new();
        }
        paths.sort_by_cached_key(|x| x.len());
        // for path in &paths {
        //     dbg!(&path.len());
        // }
        (*paths.last().expect("early return if empty")).to_vec()
    }

    /// get next step in the longest path found
    fn next_step(&mut self, start_point: (usize, usize)) -> Option<(usize, usize)> {
        self.longest_path(start_point).get(1).copied()
    }

    fn path_by_next_hop(&mut self) -> HashMap<(usize, usize), Vec<Vec<(usize, usize)>>> {
        let mut paths = HashMap::new();
        for path in &self.paths {
            if let Some(first_hope) = path.get(1) {
                let d = paths.entry(*first_hope).or_insert(Vec::new());
                d.push(path.clone());
            }
        }
        paths
    }

    fn next_most_paths(&mut self, start_point: (usize, usize), cur_dir: String) -> String {
        self.longest_paths(start_point);
        let paths = self.path_by_next_hop();
        let entry = paths.iter().max_by_key(|(_, v)| v.len());
        if let Some((step, _)) = entry {
            next_dir(start_point, *step).unwrap()
        } else {
            cur_dir
        }
    }

    /// get next direction to follow the longest path found
    fn next_dir_longest(&mut self, start_point: (usize, usize), cur_dir: String) -> String {
        if let Some(mut step) = self.next_step(start_point) {
            if !self.pathfinding_complete {
                let candidate = self.next_step_fill(start_point, &cur_dir);
                let paths = self.path_by_next_hop();
                // entry with longest path
                let entry = paths
                    .iter()
                    .max_by_key(|(_, v)| v.iter().max_by_key(|p| p.len()))
                    .unwrap();
                let max_len = self
                    .paths
                    .iter()
                    .max_by_key(|x| x.len())
                    .unwrap_or(&Vec::new())
                    .len();

                if let Some(p) = paths.get(&candidate) {
                    let cur_max_len = p
                        .iter()
                        .max_by_key(|x| x.len())
                        .unwrap_or(&Vec::new())
                        .len();
                    if max_len > cur_max_len / 2 {
                        step = candidate;
                        dbg!("OK");
                    } else {
                        step = *entry.0;
                        dbg!("path too short");
                    }
                } else {
                    step = *entry.0;
                    dbg!("no path");
                }
            }
            next_dir(start_point, step).unwrap()
        } else {
            cur_dir
        }
    }

    fn next_step_fill(&self, p: (usize, usize), cur_dir: &str) -> (usize, usize) {
        let height = self.ground.len();
        let width = self.ground[0].len();
        let dextrogyre = true;

        let coords: Vec<(usize, usize)> = match (cur_dir, dextrogyre) {
            ("LEFT", true) => {
                vec![(p.0 + 1, p.1), (p.0, p.1 - 1), (p.0 - 1, p.1)]
            }
            ("LEFT", false) => {
                vec![(p.0 - 1, p.1), (p.0, p.1 - 1), (p.0 + 1, p.1)]
            }
            ("UP", true) => {
                vec![(p.0, p.1 - 1), (p.0 - 1, p.1), (p.0, p.1 + 1)]
            }
            ("UP", false) => {
                vec![(p.0, p.1 + 1), (p.0 - 1, p.1), (p.0, p.1 - 1)]
            }
            ("RIGHT", true) => {
                vec![(p.0 - 1, p.1), (p.0, p.1 + 1), (p.0 + 1, p.1)]
            }
            ("RIGHT", false) => {
                vec![(p.0 + 1, p.1), (p.0, p.1 + 1), (p.0 - 1, p.1)]
            }
            ("DOWN", true) => {
                vec![(p.0, p.1 + 1), (p.0 + 1, p.1), (p.0, p.1 - 1)]
            }
            ("DOWN", false) => {
                vec![(p.0, p.1 + 1), (p.0 + 1, p.1), (p.0, p.1 - 1)]
            }
            _ => {
                vec![
                    (p.0 + 1, p.1),
                    (p.0 - 1, p.1),
                    (p.0, p.1 + 1),
                    (p.0, p.1 - 1),
                ]
            }
        };
        let first = (coords[0].0, coords[0].1);

        for (row, col) in coords {
            // if row == 0 || row >= height {
            if row >= height {
                continue;
            }
            // if col == 0 || col >= width {
            if col >= width {
                continue;
            }
            if self.ground[row][col] == '.' {
                return (row, col);
            }
        }
        first
    }
}

impl Display for Playground {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut lines: Vec<String> = Vec::new();

        for row in &self.ground {
            lines.push(row.iter().collect::<String>());
        }
        Display::fmt(&lines.join("\n"), f)
    }
}

/// Return next direction
fn next_dir(start_point: (usize, usize), next_point: (usize, usize)) -> Option<String> {
    if next_point.0 < start_point.0 {
        Some("UP".into())
    } else if next_point.0 > start_point.0 {
        Some("DOWN".into())
    } else if next_point.1 < start_point.1 {
        Some("LEFT".into())
    } else if next_point.1 > start_point.1 {
        Some("RIGHT".into())
    } else {
        None
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

    // playgroung
    // '.' : free space
    // '<n>' : trace let by player n
    let mut ground = Playground::new();
    let mut previous_dir = String::from("UP");

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

        // eprintln!("{}", &ground);

        // previous_dir = ground.next_dir_longest(players[p], previous_dir);
        // previous_dir = ground.next_most_paths(players[p], previous_dir);
        previous_dir = ground.next_dumb_paths(players[p], previous_dir);
        println!("{}", previous_dir);
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
            if !next_neighbours.is_empty() {
                reached.extend(next_neighbours.as_slice());
            }
        }
    }
    candidate_hop[0]
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
    None
}
