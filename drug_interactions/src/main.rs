// https://www.codingame.com/ide/puzzle/drug-interactions
//
use itertools::iproduct;
use std::collections::{HashMap, HashSet};
use std::fmt::Debug;
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
    let mut indexes = CombinationIndexes::default();
    for list_len in (0..drugs.len()).rev() {
        let comb_indexes = indexes.get_comb(list_len, drugs.len());
        dbg!(list_len, drugs.len(), comb_indexes.len());
        for comb_index in comb_indexes {
            let drug_list: Vec<String> = comb_index.iter().map(|&idx| drugs[idx].clone()).collect();

            if set_safe(&drug_list) {
                return list_len;
            }
        }
    }
    0
}

#[derive(Debug, Default)]
struct CombinationIndexes {
    results: HashMap<(usize, usize), HashSet<Vec<usize>>>,
}

impl CombinationIndexes {
    fn get_comb(&mut self, k: usize, n: usize) -> HashSet<Vec<usize>> {
        match self.results.get(&(k, n)) {
            Some(r) => return r.clone(),
            None => {}
        }

        let result: HashSet<Vec<_>> = if k == 0 {
            Default::default()
        } else if k == 1 {
            (0..n).map(|x| vec![x]).collect()
        } else if k >= n {
            [(0..n).collect()].into()
        } else {
            iproduct!(comb(k - 1, n), 0..n)
                .filter_map(|(old, new)| {
                    if old.contains(&new) {
                        None
                    } else {
                        let mut binding = old.clone();
                        binding.push(new);
                        binding.sort();
                        Some(binding)
                    }
                })
                .collect()
        };
        self.results.insert((k, n), result.clone());
        result
    }
}

fn comb(k: usize, n: usize) -> HashSet<Vec<usize>> {
    if k == 0 {
        Default::default()
    } else if k == 1 {
        (0..n).map(|x| vec![x]).collect()
    } else if k >= n {
        [(0..n).collect()].into()
    } else {
        iproduct!(comb(k - 1, n), 0..n)
            .filter_map(|(old, new)| {
                if old.contains(&new) {
                    None
                } else {
                    let mut binding = old.clone();
                    binding.push(new);
                    binding.sort();
                    Some(binding)
                }
            })
            .collect()
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
