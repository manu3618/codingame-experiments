// https://www.codingame.com/ide/puzzle/jack-silver-the-casino
use std::io;
use std::str::FromStr;

#[derive(Debug, Clone, Copy)]
enum Call {
    Plain(u32),
    Even,
    Odd,
}

#[derive(Debug)]
struct Bet {
    call: Call,
    money: u32,
}

impl Bet {
    /// Compute money earned
    fn win(&self, result: u32) -> u32 {
        match (self.call, result) {
            (Call::Plain(u), d) if d == u => self.money * 36,
            (Call::Even, 0) => 0,
            (Call::Even, d) if d % 2 == 0 => self.money * 2,
            (Call::Odd, d) if d % 2 == 1 => self.money * 2,
            _ => 0,
        }
    }
    fn with_money(&self, money: u32) -> Self {
        Self { money, ..*self }
    }
}

#[derive(Debug)]
struct ParseError;

impl FromStr for Bet {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if &s[..3] == "EVE" {
            Ok(Self {
                call: Call::Even,
                money: 0,
            })
        } else if &s[..3] == "ODD" {
            Ok(Self {
                call: Call::Odd,
                money: 0,
            })
        } else {
            let n = s[6..].parse().unwrap();
            Ok(Self {
                call: Call::Plain(n),
                money: 0,
            })
        }
    }
}

macro_rules! parse_input {
    ($x:expr, $t:ident) => {
        $x.trim().parse::<$t>().unwrap()
    };
}

/**
 * Auto-generated code below aims at helping you parse
 * the standard input according to the problem statement.
 **/
fn main() {
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let rounds = parse_input!(input_line, usize);
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let mut cash = parse_input!(input_line, u32);
    for _ in 0..rounds {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let play = input_line.trim_matches('\n').to_string();
        let parts: Vec<_> = play.splitn(2, ' ').collect();
        let ball: u32 = parts[0].parse().unwrap();
        let risk = if cash % 4 == 0 {
            cash / 4
        } else {
            cash / 4 + 1
        };
        dbg!(&cash);
        let bet: Bet = parts[1].parse::<Bet>().unwrap().with_money(risk);
        cash -= risk;
        cash += bet.win(ball);
        dbg!(&input_line);
        dbg!(&bet);
        dbg!(&cash);
    }

    // Write an answer using println!("message...");
    // To debug: eprintln!("Debug message...");

    println!("{}", cash);
}
