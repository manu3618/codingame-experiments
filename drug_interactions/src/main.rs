// https://www.codingame.com/ide/puzzle/drug-interactions
//
use itertools::iproduct;
use std::collections::{HashMap, HashSet};
use std::fmt::Debug;
use std::hash::Hash;
use std::io;

macro_rules! parse_input {
    ($x:expr, $t:ident) => {
        $x.trim().parse::<$t>().unwrap()
    };
}

// TODO: add memoization
fn safe_together(a: &str, b: &str) -> bool {
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

#[derive(Debug, Default)]
struct SafeTogether(HashMap<(String, String), bool>);

impl SafeTogether {
    fn safe_together(&mut self, a: &str, b: &str) -> bool {
        let value = self
            .0
            .entry((a.to_string(), b.to_string()))
            .or_insert(safe_together(a, b));
        *value
    }
}

/// Is the list/set of drugs safe to take together
///
/// return false if any couple of drugs is bad together
fn set_safe(drugs: &[String]) -> bool {
    let mut st = SafeTogether::default();
    let drug_couples = iproduct!(drugs, drugs)
        .map(|(d1, d2)| if d1 < d2 { (d1, d2) } else { (d2, d1) })
        .collect::<HashSet<_>>();
    drug_couples
        .into_iter()
        // .inspect(|(d1, d2)| {
        //     dbg!(d1, d2, safe_together(d1.to_string(), d2.to_string()));
        // })
        .filter(|(d1, d2)| st.safe_together(d1, d2))
        .collect::<Vec<_>>()
        .is_empty()
}

fn resolve(drugs: Vec<String>) -> usize {
    let drug_lists = Combinations::new(drugs);
    let mut previous_len = 0;
    let mut cur_len = 0;
    let mut cur_len_safe = true;
    for drug_list in drug_lists {
        cur_len = drug_list.len();

        if cur_len != previous_len {
            // new length, reset all
            if !cur_len_safe {
                return previous_len;
            }
            cur_len_safe = false;
        }

        if set_safe(&drug_list) {
            cur_len_safe = true
        }
        previous_len = drug_list.len();
    }
    if cur_len_safe { cur_len } else { cur_len - 1 }
}

#[derive(Debug)]
struct Combinations<T: Debug> {
    initial: Vec<T>,
    already_yield: Vec<Vec<T>>,
    next_yield: Vec<Vec<T>>,
}

impl<T: Debug + Clone> Combinations<T> {
    fn new(items: Vec<T>) -> Self {
        Self {
            initial: items.clone(),
            already_yield: Vec::new(),
            next_yield: items.iter().map(|x| vec![x.clone()]).collect(),
        }
    }
}

impl<T: Eq + Clone + Debug + Hash + Ord> Iterator for Combinations<T> {
    type Item = Vec<T>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.next_yield.is_empty() {
            return None;
        } else if self.next_yield.len() == 1 {
            self.already_yield.push(self.next_yield.pop().unwrap());
            for to_alter in self.already_yield.iter().rev() {
                let mut to_add: Vec<Vec<T>> = self
                    .initial
                    .iter()
                    .filter_map(|elt| {
                        if to_alter.contains(elt) {
                            None
                        } else {
                            let mut new = to_alter.clone();
                            new.push(elt.clone());
                            Some(new)
                        }
                    })
                    .filter(|x| !self.already_yield.contains(x) && !self.next_yield.contains(x))
                    .collect();
                self.next_yield.append(&mut to_add);
            }
            let binding = self
                .next_yield
                .iter()
                .map(|x| {
                    let mut a = x.clone();
                    a.sort();
                    a
                })
                .collect::<HashSet<_>>();
            self.next_yield = binding.iter().cloned().collect();
            self.next_yield.sort_by_key(|x| x.len());
        } else {
            // next_yield not empty
            let next_elt = self.next_yield.pop().unwrap();
            if next_elt.len() == self.initial.len() {
                self.next_yield.truncate(0);
            }
            self.already_yield.push(next_elt);
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
