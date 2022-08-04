// https://www.codingame.com/ide/puzzle/roller-coaster

// use cached::proc_macro::cached; // unavailble on coding game
use std::collections::HashMap;
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
    let mut waiting_queue: Vec<u64> = Vec::new();
    let mut revenue = 0;
    let mut occupied = 0; // number of places occupied
    let mut next_group: usize = 0; // index to next group
    let mut cache: HashMap<usize, (u64, usize)> = HashMap::new();
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let inputs = input_line.split(" ").collect::<Vec<_>>();
    let l = parse_input!(inputs[0], u64); // place limit
    let c = parse_input!(inputs[1], u64); // number of ime the roller coaster run
    let n = parse_input!(inputs[2], usize); //number of groupe
    for i in 0..n {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let pi = parse_input!(input_line, u64);
        waiting_queue.push(pi);
    }

    // Write an answer using println!("message...");
    // To debug: eprintln!("Debug message...");

    for round in 0..c {
        (occupied, next_group) = get_revenue(&waiting_queue, next_group, l, n, &mut cache);
        revenue += occupied;
    }
    println!("{}", revenue);
}

/// Wrapper over get_revenue_nocache
/// Quick and dirty implementation of cache mechanism
fn get_revenue(
    waiting_queue: &Vec<u64>,
    next_group: usize,
    l: u64,
    n: usize,
    cache: &mut HashMap<usize, (u64, usize)>,
) -> (u64, usize) {
    let result = cache.get(&next_group);
    match result {
        None => {
            let new_result = get_revenue_nocache(&waiting_queue.to_vec(), next_group, l, n);
            cache.insert(next_group, new_result);
            return new_result;
        }
        Some(x) => {
            // eprintln!("Debug message... use cache ({})", next_group);
            return *x;
        }
    }
}

fn get_revenue_nocache(
    waiting_queue: &Vec<u64>,
    next_group: usize,
    l: u64,
    n: usize,
) -> (u64, usize) {
    let mut occupied = 0;
    let mut nb_group_inside = 0;
    let mut group_idx = next_group;
    // &inside.clear();
    while occupied < l && nb_group_inside < waiting_queue.len() {
        // eprintln!("Debug message... putting next group");
        if waiting_queue[group_idx] > l - occupied {
            // no more room
            break;
        }
        nb_group_inside += 1;
        occupied += waiting_queue[group_idx];
        group_idx = (group_idx + 1) % n
        // eprintln!("Debug message... occupation {} / {}", occupied, l);
        // eprintln!("Debug message... waiting_queue {:?}", waiting_queue);
    }

    return (occupied, group_idx);
}
