// https://www.codingame.com/ide/puzzle/the-labyrinth
use itertools::iproduct;
use std::cmp;
use std::collections::HashMap;
use std::collections::HashSet;
use std::io;
use std::time;

macro_rules! parse_input {
    ($x:expr, $t:ident) => {
        $x.trim().parse::<$t>().unwrap()
    };
}

// #[derive(Clone, Hash)]
// struct Point {
//     row: usize,
//     col: usize,
// }

/**
 * Auto-generated code below aims at helping you parse
 * the standard input according to the problem statement.
 **/
fn main() {
    let now = time::Instant::now();
    // eprintln!("time {:?}", now.elapsed());
    let mut way_back = false;
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let inputs = input_line.split(" ").collect::<Vec<_>>();
    let r = parse_input!(inputs[0], i32); // number of rows.
    let c = parse_input!(inputs[1], i32); // number of columns.
    let a = parse_input!(inputs[2], i32); // number of rounds between the time the alarm countdown is activated and the time the alarm goes off.

    // row 0 and col 0 useless (avoid offset in coordinates)
    let mut maze = vec![vec!['?'; (c + 1) as usize]; (r + 1) as usize];
    // eprintln!("time {:?}", now.elapsed());

    // game loop
    loop {
        eprintln!("loop time {:?}", now.elapsed());
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let inputs = input_line.split(" ").collect::<Vec<_>>();
        let kr = parse_input!(inputs[0], i32); // row where Rick is located.
        let kc = parse_input!(inputs[1], i32); // column where Rick is located.
        for i in 0..r as usize {
            let mut input_line = String::new();
            io::stdin().read_line(&mut input_line).unwrap();
            // let row = input_line.trim().to_string(); // C of the characters in '#.TC?' (i.e. one line of the ASCII maze).

            // let num: String = i.to_string();
            // input_line.insert(0, num.chars().next().unwrap());
            maze[i] = input_line.chars().collect();
            eprintln!("Debug message... {}\t{:?}", i, input_line);
        }

        // Write an action using println!("message...");
        // To debug: eprintln!("Debug message...");

        // go to C
        let mut goal = (0 as usize, 0 as usize);
        if !way_back && maze[kr as usize][kc as usize] == 'C' {
            way_back = true;
        }
        if !way_back {
            goal = match get_coordinates('C', &maze) {
                Some(x) => x,
                None => get_coordinates('?', &maze).unwrap_or((0 as usize, 0 as usize)),
            }
        } else {
            goal = get_coordinates('T', &maze).unwrap();
        }
        // eprintln!("time {:?}", now.elapsed());
        eprintln!("Debug message... current position {} {}", kr, kc,);
        // eprintln!("Debug message... {:?}", &maze[kr as usize]);
        eprintln!("Debug message... {}", &maze[kr as usize][kc as usize]);
        eprintln!(
            "Debug message... neighbours {:?}",
            get_neighbours((kr as usize, kc as usize), &maze)
        );

        eprintln!("Debug message... goal {:?}", goal);
        eprintln!("time {:?}", now.elapsed());

        let next_step = get_path((kr as usize, kc as usize), goal, &maze);
        eprintln!("Debug message... next step {:?}", next_step);
        eprintln!("time {:?}", now.elapsed());

        // Rick's next move (UP DOWN LEFT or RIGHT).
        if (next_step.0 as i32) < kr {
            println!("DOWN")
        }
        if (next_step.0 as i32) > kr {
            println!("UP")
        }
        if (next_step.1 as i32) < kc {
            println!("LEFT")
        }
        if (next_step.1 as i32) > kc {
            println!("RIGHT")
        }
    }
}

/// Get coordinante of a specifi char in the maze
fn get_coordinates(c: char, maze: &Vec<Vec<char>>) -> Option<(usize, usize)> {
    let height = maze.len();
    let width = maze[0].len();
    // eprintln!("Debug message... AAAAA");
    // if maze[height - 1][width - 1] == c {}
    // eprintln!("Debug message... BBBBB");
    for (row, col) in iproduct!(0..height, 0..width) {
        if maze[row][col] == c {
            return Some((row, col));
        }
    }
    return None;
}

fn get_neighbours(p: (usize, usize), maze: &Vec<Vec<char>>) -> Vec<(usize, usize)> {
    let height = maze.len();
    let width = maze[0].len();
    let mut neighbours = Vec::new();

    let row_min = cmp::max(p.0 - 1, 0);
    let row_max = cmp::min(p.0 + 2, height);
    let col_min = cmp::max(p.1 - 1, 0);
    let col_max = cmp::min(p.1 + 2, width);

    for (row, col) in iproduct!(row_min as usize..row_max, col_min as usize..col_max) {
        if (row, col) == p {
            continue;
        }
        if maze[row][col] != '#' {
            neighbours.push((row, col));
        }
    }
    //eprintln!("Debug message... neighbours {:?} {:?}", &p, &neighbours);
    neighbours
}

/// Quick and dirty A*
/// Return next hop
fn get_path(start: (usize, usize), goal: (usize, usize), maze: &Vec<Vec<char>>) -> (usize, usize) {
    // let now = time::Instant::now();
    let mut visited = HashMap::new(); // 1st_hop : HeshSet{(uize, usize)}
    let max_cost = 100;
    let candidate_hop = get_neighbours(start, maze);
    if candidate_hop.len() == 1 {
        return candidate_hop[0];
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
                for n in get_neighbours(*r, maze).iter() {
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
