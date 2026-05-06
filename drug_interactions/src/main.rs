// https://www.codingame.com/ide/puzzle/drug-interactions
//
use itertools::iproduct;
use std::collections::HashSet;
use std::fmt::Debug;
use std::io;

macro_rules! parse_input {
    ($x:expr, $t:ident) => {
        $x.trim().parse::<$t>().unwrap()
    };
}

// TODO: add memoization
fn safe_together(a: String, b: String) -> bool {
    let a: Vec<char> = a.to_lowercase().chars().collect();
    let mut b: Vec<char> = b.to_lowercase().chars().collect();
    let mut counter = 0;
    for c in a {
        if let Some(idx) = b.iter().position(|&elt| elt == c) {
            b.swap_remove(idx);
            counter += 1;
            if counter > 2 {
                return false;
            }
        }
    }
    counter < 4
}

/// Is the list/set of drugs safe to take together
///
/// return false if any couple of drugs is bad together
fn set_safe(drugs: &[String]) -> bool {
    let drug_couples = iproduct!(drugs, drugs)
        .map(|(d1, d2)| if d1 < d2 { (d1, d2) } else { (d2, d1) })
        .collect::<HashSet<_>>();
    drug_couples
        .into_iter()
        // .inspect(|(d1, d2)| {
        //     dbg!(d1, d2, safe_together(d1.to_string(), d2.to_string()));
        // })
        .filter(|(d1, d2)| safe_together(d1.to_string(), d2.to_string()))
        .collect::<Vec<_>>()
        .is_empty()
}

fn resolve(drugs: Vec<String>) -> usize {
    // let drug_lists = get_combinations(drugs.clone());
    let drug_lists = Combinations::new(drugs);
    //dbg!(drug_lists.clone().iter().map(|c|c.len()).collect::<Vec<_>>());
    // dbg!(counter(
    //     &drug_lists.clone().iter().map(|c| c.len()).collect()
    // ));
    for drug_list in drug_lists {
        if drug_list.len() != 5 {
            //  XXX DEBUG
            // continue;
        }
        if set_safe(&drug_list) {
            dbg!(&drug_list);
            return drug_list.len();
        }
    }
    0
}

#[derive(Debug)]
struct Combinations<T: Debug> {
    already_yield: Vec<Vec<T>>,
    next_yield: Vec<Vec<T>>,
}

impl<T: Debug> Combinations<T> {
    fn new(items: Vec<T>) -> Self {
        Self {
            already_yield: Vec::new(),
            next_yield: vec![items],
        }
    }
}

impl<T: Eq + Clone + Debug> Iterator for Combinations<T> {
    type Item = Vec<T>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.next_yield.is_empty() {
            return None;
        } else if self.next_yield.len() == 1 {
            // repopulate next_yield
            self.already_yield.push(self.next_yield.pop().unwrap());
            'outer: for candidate in self.already_yield.iter().rev() {
                for idx in 0..candidate.len() {
                    let mut new_comb = candidate.clone();
                    new_comb.remove(idx);
                    if self.already_yield.contains(&new_comb) {
                        break 'outer;
                    }
                    self.next_yield.push(new_comb);
                }
            }
        } else {
            self.already_yield.push(self.next_yield.pop().unwrap());
        }
        self.already_yield.last().cloned()
    }
}

fn main() {
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let n = parse_input!(input_line, i32);
    let mut drugs = Vec::new();
    for _ in 0..n as usize {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let s = input_line.trim_matches('\n').to_string();
        drugs.push(s.clone())
    }

    // Write an answer using println!("message...");
    // To debug: eprintln!("Debug message...");
    println!("{}", resolve(drugs));
}
