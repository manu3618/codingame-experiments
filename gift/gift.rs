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
    if (&mut budgets).iter().sum::<i32>() < total {
        return None;
    }
    let mut share: Vec<i32> = Vec::new();
    let pilipu_nb = (&mut budgets).len();
    let mut idx = 0;
    for _ in 0..total {
        if (&mut share).iter().sum::<i32>() == total {
            return Some(share);
        }
        while (&mut budgets)[idx] == 0 {
            idx = (idx + 1) % pilipu_nb;
        }
        share[idx] = share[idx] + 1;
        budgets[idx] = budgets[idx] - 1;
    }
    return None;
}
