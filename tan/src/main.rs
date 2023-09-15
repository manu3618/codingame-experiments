// https://www.codingame.com/ide/puzzle/tan-network
use std::collections::HashMap;
use std::collections::HashSet;
use std::io;
use std::str::FromStr;

macro_rules! parse_input {
    ($x:expr, $t:ident) => {
        $x.trim().parse::<$t>().unwrap()
    };
}

#[derive(Debug, PartialEq, Default, Clone)]
struct Stop {
    id: String,
    name: String,
    latitude: f64,
    longitude: f64,
}

#[derive(Debug)]
struct TanError;

impl FromStr for Stop {
    type Err = TanError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let fields: Vec<&str> = s.split(',').collect();
        match fields.len() {
            0..=4 => Err(TanError),
            _ => Ok(Self {
                id: String::from(*fields.get(0).unwrap()),
                name: String::from(*fields.get(1).unwrap())
                    .trim_matches('"')
                    .into(),
                latitude: fields
                    .get(3)
                    .unwrap()
                    .parse()
                    .expect("latitude should be a number"),
                longitude: fields
                    .get(4)
                    .unwrap()
                    .parse()
                    .expect("longitude should be a number"),
            }),
        }
    }
}

impl Stop {
    fn distance(&self, other: &Self) -> f64 {
        let x = (self.longitude - other.longitude) * ((self.latitude + other.latitude) / 2.0).cos();
        let y = self.latitude - other.latitude;
        (x.powi(2) + y.powi(2)).sqrt() * 6371.0
    }
}

#[derive(Debug, Default)]
struct Network {
    links: HashMap<String, Vec<Link>>, // src: [dst]
    stops: HashMap<String, Stop>,
}

#[derive(Debug, PartialEq, Default)]
struct Link {
    src: String,
    dst: String,
    distance: f64,
}

impl Network {
    fn add_link(&mut self, src: &str, dst: &str) {
        let src_stop = self
            .stops
            .get(src.into())
            .expect(&format!("stop {} not in network", src));
        let dst_stop = self
            .stops
            .get(dst.into())
            .expect(&format!("stop {} not in network", dst));
        let link = Link {
            src: src.into(),
            dst: dst.into(),
            distance: src_stop.distance(dst_stop),
        };
        let destinations = self.links.entry(src.into()).or_insert(Vec::new());
        if !destinations.contains(&link) {
            destinations.push(link);
        }
    }
    fn insert_stop(&mut self, stop: Stop) {
        self.stops.insert(stop.id.clone(), stop);
    }
}

/**
 * Auto-generated code below aims at helping you parse
 * the standard input according to the problem statement.
 **/
fn main() {
    let mut input_line = String::new();
    let mut network = Network::default();
    io::stdin().read_line(&mut input_line).unwrap();
    let start_point = input_line.trim().to_string();
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let end_point = input_line.trim().to_string();
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let n = parse_input!(input_line, i32);
    for i in 0..n as usize {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let stop_name = input_line.trim_matches('\n').to_string();
        let stop: Stop = stop_name.parse().expect("input not compatible");
        network.insert_stop(stop);
    }
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let m = parse_input!(input_line, i32);
    for i in 0..m as usize {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let route = input_line.trim_matches('\n').to_string();
        let link: Vec<&str> = route.split(' ').collect();
        network.add_link(link[0], link[1]);
    }

    dbg!(network);
    // Write an answer using println!("message...");
    // To debug: eprintln!("Debug message...");

    println!("IMPOSSIBLE");
}
