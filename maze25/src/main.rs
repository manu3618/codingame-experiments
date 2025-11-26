// https://www.codingame.com/ide/puzzle/2-5d-maze

use std::collections::HashMap;
use std::io;
use std::str::FromStr;

macro_rules! parse_input {
    ($x:expr, $t:ident) => {
        $x.trim().parse::<$t>().unwrap()
    };
}

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
enum Floor {
    Lower,
    Upper,
}

impl Floor {
    fn invert(&self) -> Self {
        match &self {
            Self::Upper => Self::Lower,
            Self::Lower => Self::Upper,
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Direction {
    North,
    South,
    East,
    West,
}

#[derive(Debug, Default)]
enum Cell {
    HorizontalSlope,
    VerticalSlope,
    /// wall or empty space
    #[default]
    Unwalkable,
    /// any other cell
    Walkable,
}
impl Cell {
    fn parse_floor(c: char, floor: Floor) -> Cell {
        match (c, floor) {
            ('.', Floor::Upper) => Self::Unwalkable,
            ('.', Floor::Lower) => Self::Walkable,
            ('+', Floor::Upper) => Self::Walkable,
            ('+', Floor::Lower) => Self::Unwalkable,
            ('-', _) => Self::HorizontalSlope,
            ('|', _) => Self::VerticalSlope,
            ('#', _) => Self::Unwalkable,
            ('X', _) => Self::Walkable,
            ('O', Floor::Upper) => Self::Unwalkable,
            ('O', Floor::Lower) => Self::Walkable,
            (_, _) => unreachable!(),
        }
    }
}

#[derive(Debug)]
struct Maze {
    /// (number of lines, number of columns)
    size: (usize, usize),
    /// Vec of lines
    lower_floor: Vec<Vec<Cell>>,
    upper_floor: Vec<Vec<Cell>>,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
struct Coord {
    line: usize,
    column: usize,
    floor: Floor,
}

impl Coord {
    fn from_tuple(a: (usize, usize)) -> Self {
        Self {
            line: a.0,
            column: a.1,
            floor: Floor::Lower,
        }
    }
}

#[derive(Debug, Default, Clone)]
struct Path(Vec<Coord>);

impl Path {
    fn cost(&self) -> usize {
        self.0.len()
    }
}

#[derive(Debug, PartialEq, Eq)]
struct ParseMazeError(String);

impl FromStr for Maze {
    type Err = ParseMazeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines();
        let Some((h, w)) = lines.next().unwrap().split_once(" ") else {
            return Err(ParseMazeError("unable to parse size".into()));
        };
        let Ok(h) = h.parse() else {
            return Err(ParseMazeError("unable to parse height".into()));
        };
        let Ok(w) = w.parse() else {
            return Err(ParseMazeError("unable to parse width".into()));
        };

        let mut lower_floor = Vec::with_capacity(h);
        let mut upper_floor = Vec::with_capacity(h);

        for l in lines {
            dbg!(&l);
            let lower_line = l
                .chars()
                .map(|c| Cell::parse_floor(c, Floor::Lower))
                .collect::<Vec<_>>();
            let upper_line = l
                .chars()
                .map(|c| Cell::parse_floor(c, Floor::Upper))
                .collect::<Vec<_>>();

            lower_floor.push(lower_line);
            upper_floor.push(upper_line);
        }

        Ok(Self {
            size: (h, w),
            lower_floor,
            upper_floor,
        })
    }
}

impl Maze {
    fn get(&self, c: Coord) -> Option<&Cell> {
        match c.floor {
            Floor::Upper => self.upper_floor.get(c.line)?.get(c.column),
            Floor::Lower => self.lower_floor.get(c.line)?.get(c.column),
        }
    }

    /// Compute shortest path
    /// Path is returned as list of coordinates.
    fn get_path(&self, start: (usize, usize), end: (usize, usize)) -> Path {
        let mut paths: HashMap<Coord, Path> = HashMap::new();
        let start = Coord::from_tuple(start);
        let end = Coord::from_tuple(end);
        paths.insert(start.clone(), Path(vec![start]));

        while !paths.contains_key(&end) {
            let mut to_add = Vec::new();
            for p in paths.values() {
                for e in self
                    .get_path_extensions(p.0.last().expect("path contains at least start").clone())
                {
                    let dest = e.last().unwrap();
                    if p.0.contains(&dest) {
                        continue;
                    }
                    let mut new_path = p.clone();
                    new_path.0.append(&mut e.clone());
                    to_add.push(new_path);
                }
            }
            for p in to_add {
                let dest = p.0.last().expect("path not empty").clone();
                let cur = paths.entry(dest.clone()).or_insert(p.clone());
                if cur.cost() > p.cost() {
                    paths.insert(dest.clone(), p.clone());
                }
            }
        }
        paths.get(&end).expect("while loop has ended").clone()
    }

    /// get smallest possible extensions for a path ending with the coordinate c
    fn get_path_extensions(&self, c: Coord) -> Vec<Vec<Coord>> {
        let mut ret = Vec::new();
        for (dir, neigh) in self.get_neighbors(c) {
            match (dir, self.get(neigh.clone())) {
                (_, Some(Cell::Walkable)) => ret.push(vec![neigh.clone()]),
                (Direction::East, Some(Cell::HorizontalSlope)) => ret.push(vec![
                    neigh.clone(),
                    Coord {
                        column: neigh.column + 1,
                        floor: neigh.floor.invert(),
                        ..neigh
                    },
                ]),
                (Direction::West, Some(Cell::HorizontalSlope)) => ret.push(vec![
                    neigh.clone(),
                    Coord {
                        column: neigh.column - 1,
                        floor: neigh.floor.invert(),
                        ..neigh
                    },
                ]),

                (Direction::North, Some(Cell::VerticalSlope)) => ret.push(vec![
                    neigh.clone(),
                    Coord {
                        line: neigh.line - 1,
                        floor: neigh.floor.invert(),
                        ..neigh
                    },
                ]),

                (Direction::South, Some(Cell::VerticalSlope)) => ret.push(vec![
                    neigh.clone(),
                    Coord {
                        line: neigh.line + 1,
                        floor: neigh.floor.invert(),
                        ..neigh
                    },
                ]),

                _ => continue,
            }
        }
        ret
    }

    fn get_neighbors(&self, c: Coord) -> Vec<(Direction, Coord)> {
        let mut neighb = Vec::new();
        if c.column > 0 {
            neighb.push((
                Direction::West,
                Coord {
                    column: c.column - 1,
                    ..c
                },
            ));
        }
        if c.column < self.size.1 {
            neighb.push((
                Direction::East,
                Coord {
                    column: c.column + 1,
                    ..c
                },
            ));
        }
        if c.line > 0 {
            neighb.push((
                Direction::North,
                Coord {
                    line: c.line - 1,
                    ..c
                },
            ));
        }
        if c.line < self.size.0 {
            neighb.push((
                Direction::South,
                Coord {
                    line: c.line + 1,
                    ..c
                },
            ));
        }
        neighb
    }
}

/**
 * Auto-generated code below aims at helping you parse
 * the standard input according to the problem statement.
 **/
fn main() {
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let inputs = input_line.split(" ").collect::<Vec<_>>();
    let starty = parse_input!(inputs[0], usize);
    let startx = parse_input!(inputs[1], usize);
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let inputs = input_line.split(" ").collect::<Vec<_>>();
    let endy = parse_input!(inputs[0], usize);
    let endx = parse_input!(inputs[1], usize);
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();

    let mut maze = String::new();
    let inputs = input_line.split(" ").collect::<Vec<_>>();
    maze.push_str(input_line.as_str());
    let h = parse_input!(inputs[0], i32);
    // let w = parse_input!(inputs[1], i32);
    for _ in 0..h as usize {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let line = input_line.trim().to_string();
        maze.push_str(format!("{line}\n").as_str())
    }
    let maze = Maze::from_str(maze.as_str().trim()).unwrap();

    // Write an answer using println!("message...");
    // To debug: eprintln!("Debug message...");

    println!(
        "{}",
        maze.get_path((starty, startx), (endy, endx)).cost() - 1
    );
}
