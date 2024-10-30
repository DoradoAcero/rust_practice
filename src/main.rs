mod strats;
mod wordle_utils;

use std::fs;

use strats::{my_strat, run_basic_strat, stake_first_strat};


fn main() {
    let iter_count = 1000;
    let file_path = "words.txt";

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");
    
    let words: Vec<&str> = contents.lines().collect();

    let strats_functions = vec![run_basic_strat, stake_first_strat, my_strat];

    // there is the possibility of some sick meta programming here, but ill park that for now
    for strat_function in strats_functions {
        let mut counts = vec![];
        for _ in 0..iter_count {
            counts.insert(0, strat_function(words.clone()));
        };

        let sum: i32 = counts.iter().sum();
        let average = sum as f64 / counts.len() as f64;
        println!("Average guess count: {}", average);
        println!("------------------------------------------------------------");
    }
}