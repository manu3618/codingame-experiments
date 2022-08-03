use std::collections::VecDeque;
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
    let mut waiting_queue: VecDeque<i64> = VecDeque::new();
    let mut revenue = 0;
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let inputs = input_line.split(" ").collect::<Vec<_>>();
    let l = parse_input!(inputs[0], i64); // place limit
    let c = parse_input!(inputs[1], i64); // number of ime the roller coaster run
    let n = parse_input!(inputs[2], i64); //number of groupe
    for i in 0..n as usize {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let pi = parse_input!(input_line, i64);
        waiting_queue.push_back(pi);
    }

    // Write an answer using println!("message...");
    // To debug: eprintln!("Debug message...");

    for round in 0..c {
        // eprintln!("Debug message... nround {}", round);
        // eprintln!("Debug message... waiting_queue {:?}", waiting_queue);
        let mut occupied = 0; // number of places occupied
        let mut inside: Vec<i64> = Vec::new(); // gourps inside
        while occupied < l && waiting_queue.len() > 0 {
            // look at next group
            if waiting_queue.get(0).unwrap() > &(l - occupied) {
                // unabale to put next group
                // eprintln!("Debug message... not enough room for next group");
                break;
            }
            // eprintln!("Debug message... putting next group");
            inside.push(waiting_queue.pop_front().unwrap());
            occupied += inside[inside.len() - 1];
            // eprintln!("Debug message... occupation {} / {}", occupied, l);
            // eprintln!("Debug message... waiting_queue {:?}", waiting_queue);
        }
        revenue += occupied;

        // finish turn
        for pi in inside {
            waiting_queue.push_back(pi)
        }
        // eprintln!("Debug message... end of round {}", round);
        // eprintln!("Debug message... waiting_queue {:?}", waiting_queue);
    }
    println!("{}", revenue)
}
