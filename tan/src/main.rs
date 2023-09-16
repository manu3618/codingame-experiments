// https://www.codingame.com/ide/puzzle/tan-network
use std::cmp::Ordering;
use std::collections::HashMap;
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

#[derive(Debug, PartialEq, Default, Clone)]
struct Route {
    route: Vec<String>, // stops ids in order
    distance: f64,
}

impl Route {
    fn add_link(&mut self, link: &Link) {
        if self.route.is_empty() {
            self.route.push(link.src.clone());
        }
        if self.route.contains(&link.dst) {
            return;
        }
        if self.route.last().expect("route contains at least source") != &link.src {
            return;
        }
        self.route.push(link.dst.clone());
        self.distance += link.distance;
    }
}

impl PartialOrd for Route {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.distance.partial_cmp(&other.distance)
    }
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

    /// Return all possible route with exactly one more hope.
    fn extend_route(&self, route: Route) -> Vec<Route> {
        let mut routes = Vec::new();
        if route.route.is_empty() {
            return routes;
        }
        let src = route.route.last().expect("route not empty").clone();
        if let Some(links) = self.links.get(&src) {
            for link in links {
                let mut new_route = route.clone();
                new_route.add_link(link);
                routes.push(new_route);
            }
        }
        return routes;
    }

    /// find all routes from src_id to dst_id
    fn build_routes(&self, src_id: &str, dst_id: &str) -> Vec<Route> {
        if src_id == dst_id {
            return vec![Route {
                route: vec![String::from(src_id)],
                distance: 0.0,
            }];
        }
        let mut route_modified = true; // whether a route has been modified
        let mut routes = Vec::new();
        if let Some(links) = self.links.get(src_id) {
            for link in links {
                let mut route = Route::default();
                route.add_link(link);
                routes.push(route);
            }
        }
        let mut new_routes = Vec::new();
        while route_modified {
            route_modified = false;
            for route in &routes {
                if route.route.last().expect("route is not empty") == dst_id {
                    // destination reached
                    continue;
                }
                let new_hopes = self.extend_route(route.clone());
                if new_hopes.is_empty() {
                    // no new route
                    continue;
                }
                route_modified = true;
                // dbg!(&new_hopes);
                new_routes.extend(new_hopes);
            }
            routes.append(&mut new_routes);
        }
        routes.retain(|r| !r.route.is_empty() && r.route.last().unwrap() == dst_id);
        routes
    }

    /// Build all shortest routes from src_id
    fn build_all_routes(&self, src_id: &str) -> HashMap<String, Route> {
        let mut shortest = HashMap::new();
        shortest.insert(
            String::from(src_id),
            Route {
                route: vec![String::from(src_id)],
                distance: 0.0,
            },
        );

        let mut new_routes = Vec::new();
        let mut prev_len = 0_usize;
        while prev_len != shortest.len() {
            prev_len = shortest.len();
            dbg!(shortest.len());
            for route in shortest.values() {
                let _ = &new_routes.extend(self.extend_route(route.clone()));
            }
            for route in &new_routes {
                let dst = route.route.last().expect("route is not empty");
                if let Some(current_route) = shortest.get_mut(dst) {
                    if current_route.distance > route.distance {
                        *current_route = route.clone(); // TODO use std::mem::copy
                    }
                } else {
                    shortest.insert(String::from(dst), route.clone());
                }
            }
        }
        shortest
    }

    /// get the answer to write.
    fn get_answer(&self, src_id: &str, dst_id: &str) -> String {
        let paths = self.build_all_routes(src_id);
        match paths.get(dst_id) {
            None => String::from("IMPOSSIBLE"),
            Some(route) => {
                let mut human_readable = Vec::new();
                for stop_id in &route.route {
                    human_readable.push(self.stops.get(stop_id).unwrap().name.clone());
                }
                human_readable.join("\n")
            }
        }
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
    for _i in 0..n as usize {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let stop_name = input_line.trim_matches('\n').to_string();
        let stop: Stop = stop_name.parse().expect("input not compatible");
        network.insert_stop(stop);
    }
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let m = parse_input!(input_line, i32);
    for _i in 0..m as usize {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let route = input_line.trim_matches('\n').to_string();
        let link: Vec<&str> = route.split(' ').collect();
        network.add_link(link[0], link[1]);
    }

    // dbg!(&network);
    // Write an answer using println!("message...");
    // To debug: eprintln!("Debug message...");
    // let routes = network.build_routes(&start_point, &end_point);
    // dbg!(&routes);
    println!("{}", network.get_answer(&start_point, &end_point));
}
