// https://www.codingame.com/ide/puzzle/surface
use std::collections::HashMap;
use std::collections::HashSet;
use std::fmt;
use std::io;
use std::iter::FromIterator;

macro_rules! parse_input {
    ($x:expr, $t:ident) => {
        $x.trim().parse::<$t>().unwrap()
    };
}

#[derive(Default, Debug)]
struct WaterMap {
    height: usize,
    width: usize,
    map: Vec<Vec<char>>,
    results: HashMap<(usize, usize), usize>,
}

impl WaterMap {
    fn get_neighbors(&self, coords: (usize, usize)) -> HashSet<(usize, usize)> {
        let mut res = HashSet::with_capacity(4);
        if coords.0 > 0 {
            res.insert((coords.0 - 1, coords.1));
        }
        if coords.0 < self.height - 1 {
            res.insert((coords.0 + 1, coords.1));
        }
        if coords.1 > 0 {
            res.insert((coords.0, coords.1 - 1));
        }
        if coords.1 < self.width - 1 {
            res.insert((coords.0, coords.1 + 1));
        }
        res
    }

    fn get_water_surface(&mut self, coords: (usize, usize)) -> usize {
        if self.map[coords.0][coords.1] == '#' {
            return 0;
        }
        if let Some(&n) = self.results.get(&coords) {
            return n;
        }
        let mut area: HashSet<(usize, usize)> = HashSet::from_iter(vec![coords]);
        let mut to_explore = self.get_neighbors(coords);
        while !to_explore.is_empty() {
            let mut new_explore = HashSet::new(); // to_explore for next iteration
            for n in to_explore {
                // add to area if needed
                match self.map[n.0][n.1] {
                    '#' => continue,
                    'O' => area.insert(n),
                    _ => unreachable!(),
                };

                // extend area to explore for next iteration
                new_explore.extend(self.get_neighbors(n).iter().filter(|c| !area.contains(c)));
            }
            to_explore = new_explore;
        }

        // memoization
        let result = area.len();
        for c in area {
            self.results.insert(c, result);
        }
        result
    }
}

impl fmt::Display for WaterMap {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            self.map
                .iter()
                .map(|l| l.iter().collect::<String>())
                .collect::<Vec<_>>()
                .join("\n"),
        )
    }
}

impl FromIterator<String> for WaterMap {
    fn from_iter<I: IntoIterator<Item = String>>(iter: I) -> Self {
        let map: Vec<_> = iter
            .into_iter()
            .map(|l| l.chars().collect::<Vec<char>>())
            .collect();
        let width = map.iter().map(|l| l.len()).min().unwrap_or(0);
        WaterMap {
            height: map.len(),
            width,
            map: map
                .iter()
                .map(|l| l[..width].iter().copied().clone().collect::<Vec<_>>())
                .collect::<Vec<_>>()
                .clone(),
            results: HashMap::new(),
        }
    }
}

/**
 * Auto-generated code below aims at helping you parse
 * the standard input according to the problem statement.
 **/
fn main() {
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let _width = parse_input!(input_line, i32);
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let height = parse_input!(input_line, i32);
    let mut map_input = Vec::new();
    for _ in 0..height as usize {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let row = input_line.trim_matches('\n').to_string();
        map_input.push(row);
    }
    let mut water_map = WaterMap::from_iter(map_input);
    // eprintln!("map");
    // eprintln!("{}", &water_map);
    dbg!(&water_map.width);
    dbg!(&water_map.height);
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let n = parse_input!(input_line, i32);
    for _ in 0..n as usize {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let inputs = input_line.split(' ').collect::<Vec<_>>();
        let y = parse_input!(inputs[0], usize);
        let x = parse_input!(inputs[1], usize);

        // Write an answer using println!("message...");
        // To debug: eprintln!("Debug message...");
        println!("{}", water_map.get_water_surface((x, y)));
    }
}
