// https://www.codingame.com/ide/puzzle/wordle
use std::io;

macro_rules! parse_input {
    ($x:expr, $t:ident) => {
        $x.trim().parse::<$t>().unwrap()
    };
}

fn mask(truth: &str, proposal: &str) -> Vec<usize> {
    debug_assert!(truth.len() == 6);
    debug_assert!(proposal.len() == 6);
    let mut result = vec![1; 6];
    let mut truth: Vec<char> = truth.chars().collect();
    let mut proposal: Vec<char> = proposal.chars().collect();

    // correct
    for idx in 0..6 {
        if truth[idx] == proposal[idx] {
            proposal[idx] = '.'; // consume the used char
                                 //truth[idx] = '_'; // consume used character
            result[idx] = 3;
        }
    }

    // misplaced
    for idx in 0..6 {
        if let Some(p) = truth.iter().position(|a| a == &proposal[idx]) {
            //truth[p] = '*'; // consume used character
            result[idx] = 2;
        }
    }
    result
}

/// given the truth and state, is the proposal possible?
fn is_possible(truth: &str, proposal: &str, state: &Vec<usize>) -> bool {
    &mask(truth, proposal) == state
}

fn main() {
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let _word_count = parse_input!(input_line, usize); // Number of words in the word set
    let mut inputs = String::new();
    io::stdin().read_line(&mut inputs).unwrap();
    let mut words = inputs.split_whitespace().collect::<Vec<&str>>();

    assert!(words.contains(&"ANSWER"));
    // game loop
    loop {
        dbg!(&words.len());
        let candidate = words[0];
        let mut inputs = String::new();
        io::stdin().read_line(&mut inputs).unwrap();
        let state: Vec<usize> = inputs
            .split_whitespace()
            .map(|s| s.trim().parse().expect("should be parsable"))
            .collect();
        dbg!(state
            .iter()
            .map(|c| format!("{c}"))
            .collect::<Vec<_>>()
            .join(""));
        if !state.contains(&0) {
            words.retain(|&w| is_possible(w, candidate, &state));
            //words.retain(|&w| is_possible(candidate, w, &state));
        }
        let candidate = words[0];
        dbg!(&candidate);
        println!("{}", &candidate);
        if words.len() < 20 {
            dbg!(&words);
        }
    }
}
