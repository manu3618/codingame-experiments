// https://www.codingame.com/ide/puzzle/the-labyrinth
use itertools::iproduct;
use rand::Rng;
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

static mut init_search: (usize, usize) = (0, 0);
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
    let mut loop_nb = 0;
    // eprintln!("time {:?}", now.elapsed());

    // game loop
    loop {
        loop_nb += 1;
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
            let mut goal_char = '?';
            if loop_nb > 1000 {
                goal_char = 'C';
            }
            goal = match get_coordinates(goal_char, &maze) {
                Some(x) => x,
                None => {
                    if loop_nb % 10 == 0 {
                        // change initial research point
                        unsafe {
                            init_search.0 += 1;
                            init_search.1 += 1;
                            if init_search.0 >= r as usize {
                                init_search.0 = 0
                            }
                            if init_search.1 >= c as usize {
                                init_search.1 = 0
                            }
                        }
                    }
                    get_coordinates('?', &maze).unwrap_or((0 as usize, 0 as usize))
                }
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
            println!("UP")
        }
        if (next_step.0 as i32) > kr {
            println!("DOWN")
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
    let mut row_range: Vec<usize> = (0..height).collect();
    let mut col_range: Vec<usize> = (0..width).collect();

    // begin search eslwhere so that if there is several 'c', choose
    // another another one
    // row_range.rotate_left(rand::thread_rng().gen_range(1..=height));
    // col_range.rotate_left(rand::thread_rng().gen_range(1..=width));
    unsafe {
        row_range.rotate_left(init_search.0);
        col_range.rotate_left(init_search.1);
    }

    for (row, col) in iproduct!(row_range, col_range) {
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
