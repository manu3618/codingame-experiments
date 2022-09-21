// https://www.codingame.com/ide/puzzle/the-gift
use std::io;

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
    let n = parse_input!(input_line, i32); // pilipiu number
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let c = parse_input!(input_line, i32); // gift price
    let mut budgets: Vec<i32> = Vec::new();
    for i in 0..n as usize {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let b = parse_input!(input_line, i32);
        budgets.push(b)
    }
    eprintln!("Debug message... budgets {:?}", &budgets);
    match balance_split(&mut budgets, c) {
        None => {
            println!("IMPOSSIBLE");
        }
        Some(mut share) => {
            share.sort();
            for elt in share {
                println!("{}", elt);
            }
        }
    }
}

fn balance_split(budgets: &mut Vec<i32>, total: i32) -> Option<Vec<i32>> {
    if budgets.iter().sum::<i32>() < total {
        return None;
    }
    let pilipu_nb = budgets.len();
    let mut share: Vec<i32> = Vec::new();
    for _ in 0..pilipu_nb {
        share.push(0);
    }
    let mut idx = 0;
    for _ in 0..total + 1 {
        // eprintln!("Debug message... budgets {:?}", &budgets);
        // eprintln!("Debug message... share   {:?}", &mut share);
        if (&mut share).iter().sum::<i32>() == total {
            return Some(share);
        }
        while budgets[idx] == 0 {
            idx = (idx + 1) % pilipu_nb;
        }
        share[idx] = share[idx] + 1;
        budgets[idx] = budgets[idx] - 1;
        idx = (idx + 1) % pilipu_nb;
    }
    return None;
}
