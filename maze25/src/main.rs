use std::io;
use std::str::FromStr;

macro_rules! parse_input {
    ($x:expr, $t:ident) => {
        $x.trim().parse::<$t>().unwrap()
    };
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

enum Floor {
    Lower,
    Upper,
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

#[derive(Debug, PartialEq, Eq)]
struct ParseMazeError(String);

impl FromStr for Maze {
    type Err = ParseMazeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines();
        let Some((h, w)) = lines.next().unwrap().split_once(" ") else {
            return Err(ParseMazeError("unable to parse size".into()));
        };
        let Ok(h) = h.parse() else { todo!() };
        let Ok(w) = w.parse() else { todo!() };

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
            lower_floor: lower_floor,
            upper_floor: upper_floor,
        })
    }
}

impl Maze {
    /// Compute shortest path
    /// Path is returned as list of coordinates.
    fn get_path(&self, start: (usize, usize), end: (usize, usize)) -> Vec<(usize, usize, Floor)> {
        // let paths = HashMap
        todo!()
    }

    /// get neighbors of a cell
    fn get_neighbors(
        &self,
        line: usize,
        column: usize,
        floor: Floor,
    ) -> Vec<(usize, usize, Floor)> {
        todo!()
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
    let starty = parse_input!(inputs[0], i32);
    let startx = parse_input!(inputs[1], i32);
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let inputs = input_line.split(" ").collect::<Vec<_>>();
    let endy = parse_input!(inputs[0], i32);
    let endx = parse_input!(inputs[1], i32);
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

    println!("answer");
}
