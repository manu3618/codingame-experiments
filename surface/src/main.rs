// https://www.codingame.com/ide/puzzle/surface
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
            width: width,
            map: map
                .iter()
                .map(|l| (&l[..width].iter().map(|a| *a)).clone().collect::<Vec<_>>())
                .collect::<Vec<_>>()
                .clone(),
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
    let water_map = WaterMap::from_iter(map_input);
    eprintln!("map");
    eprintln!("{}", &water_map); // XXX
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let n = parse_input!(input_line, i32);
    for i in 0..n as usize {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let inputs = input_line.split(" ").collect::<Vec<_>>();
        let x = parse_input!(inputs[0], i32);
        let y = parse_input!(inputs[1], i32);
    }
    for i in 0..n as usize {
        // Write an answer using println!("message...");
        // To debug: eprintln!("Debug message...");

        println!("answer");
    }
}
