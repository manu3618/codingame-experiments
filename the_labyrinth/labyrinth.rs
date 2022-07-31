// https://www.codingame.com/ide/puzzle/the-labyrinth
use itertools::iproduct;
use std::cmp;
use std::collections::HashMap;
use std::io;
use std::time;

macro_rules! parse_input {
    ($x:expr, $t:ident) => {
        $x.trim().parse::<$t>().unwrap()
    };
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Copy)]
struct Point {
    row: usize,
    col: usize,
}

/**
 * Auto-generated code below aims at helping you parse
 * the standard input according to the problem statement.
 **/
fn main() {
    let now = time::Instant::now();
    let way_back = false;
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let inputs = input_line.split(" ").collect::<Vec<_>>();
    let r = parse_input!(inputs[0], i32); // number of rows.
    let c = parse_input!(inputs[1], i32); // number of columns.
    let a = parse_input!(inputs[2], i32); // number of rounds between the time the alarm countdown is activated and the time the alarm goes off.

    // row 0 and col 0 useless (avoid offset in coordinates)
    let mut maze = vec![vec!['M'; (c + 2) as usize]; (r + 2) as usize];

    // game loop
    loop {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let inputs = input_line.split(" ").collect::<Vec<_>>();
        let kr = parse_input!(inputs[0], i32); // row where Rick is located.
        let kc = parse_input!(inputs[1], i32); // column where Rick is located.
        for i in 0..(r + 0) as usize {
            let mut input_line = String::new();
            io::stdin().read_line(&mut input_line).unwrap();
            // let row = input_line.trim().to_string(); // C of the characters in '#.TC?' (i.e. one line of the ASCII maze).

            // let num: String = i.to_string();
            // input_line.insert(0, num.chars().next().unwrap());
            maze[i] = input_line.chars().collect();
            // eprintln!("Debug message... {}\t{:?}", i, input_line);
        }

        // Write an action using println!("message...");
        // To debug: eprintln!("Debug message...");

        // go to C
        let mut goal = Point { row: 0, col: 0 };
        if !way_back {
            goal = match get_coordinates('C', &maze) {
                Some(x) => x,
                None => get_coordinates('?', &maze).unwrap_or(Point { row: 0, col: 0 }),
            }
        } else {
            goal = get_coordinates('T', &maze).unwrap();
        }
        eprintln!("time {:?}", now.elapsed());
        eprintln!("Debug message... current position {} {}", kr, kc,);
        // eprintln!("Debug message... {:?}", &maze[kr as usize]);
        eprintln!("Debug message... {}", &maze[kr as usize][kc as usize]);
        eprintln!(
            "Debug message... neighbours {:?}",
            get_neighbours(
                &Point {
                    row: kr as usize,
                    col: kc as usize
                },
                &maze
            )
        );

        eprintln!("Debug message... goal {:?}", goal);

        let next_step = get_path(
            &Point {
                row: kr as usize,
                col: kc as usize,
            },
            &goal,
            &maze,
        );
        eprintln!("Debug message... next step {:?}", next_step);

        // Rick's next move (UP DOWN LEFT or RIGHT).
        if (next_step.row as i32) < kr {
            println!("LEFT")
        }

        if (next_step.row as i32) > kr {
            println!("RIGHT")
        }
        if (next_step.col as i32) < kc {
            println!("DOWN")
        }
        if (next_step.col as i32) > kc {
            println!("UP")
        }
    }
}

/// Get coordinante of a specifi char in the maze
fn get_coordinates(c: char, maze: &Vec<Vec<char>>) -> Option<Point> {
    let height = maze.len();
    let width = maze[0].len();
    // eprintln!("Debug message... AAAAA");
    // if maze[height - 1][width - 1] == c {}
    // eprintln!("Debug message... BBBBB");
    for (row, col) in iproduct!(0..height, 0..width) {
        if maze[row][col] == c {
            return Some(Point { row: row, col: col });
        }
    }
    return None;
}

fn get_neighbours(p: &Point, maze: &Vec<Vec<char>>) -> Vec<Point> {
    let height = maze.len();
    let width = maze[0].len();
    if p.row > width || p.col > height {
        return Vec::new();
    }
    let mut neighbours = Vec::new();

    let row_min = cmp::max(p.row - 1, 0);
    let row_max = cmp::min(p.row + 2, width + 1);
    let col_min = cmp::max(p.col - 1, 0);
    let col_max = cmp::min(p.col + 2, height + 1);

    for (row, col) in iproduct!(row_min as usize..row_max, col_min as usize..col_max) {
        if row == p.row && col == p.col {
            continue;
        }
        if maze[row][col] != '#' {
            neighbours.push(Point { row, col });
        }
    }
    // eprintln!("Debug message... neighbours {:?} {:?}", &p, &neighbours);
    neighbours
}

/// Quick and dirty A*
/// Return next hop
fn get_path(start: &Point, goal: &Point, maze: &Vec<Vec<char>>) -> Point {
    let now = time::Instant::now();
    let mut visited = HashMap::new(); // 1st_hop : vec[point]
    let max_cost = 100;

    let candidate_hop = get_neighbours(start, maze);
    for hop in candidate_hop.iter() {
        visited.insert(hop.clone(), vec![hop.clone()]);
    }
    // eprintln!("Debug message... init visited {:?}", &visited);

    // explore one more level of neighbors
    for cost in 0..max_cost {
        eprintln!("time  since get_path{:?}", now.elapsed());
        eprintln!("Debug message... cost {}", cost);
        for (&candidate, reached) in visited.iter_mut() {
            let mut next_neighbours: Vec<Point> = Vec::new();
            let reached_copy = reached.clone(); // copy for latter move
            for r in reached_copy.into_iter() {
                for n in get_neighbours(&r, maze).iter() {
                    if n == goal {
                        return candidate.clone();
                    }
                    if !reached.contains(&n) {
                        next_neighbours.push(n.clone());
                    }
                }
            }
            reached.extend_from_slice(next_neighbours.as_slice());
            eprintln!("Debug message... candidate {:?}", &candidate);
            eprintln!("Debug message... reached {:?}", &reached);
        }
    }
    return candidate_hop[0].clone();
}
