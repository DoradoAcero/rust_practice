mod strats;
mod wordle_utils;

use std::{borrow::Borrow, fs, sync::{Arc, LazyLock, Mutex}, thread};

use strats::{my_strat, run_basic_strat, stake_first_strat};


fn main() {
    let iter_count = 1000;
    static FILE_PATH: &str = "words.txt";

    static CONTENTS: LazyLock<String> = std::sync::LazyLock::new(|| {fs::read_to_string(FILE_PATH)
        .expect("Should have been able to read the file")});
    
    let strats_functions = vec![run_basic_strat, stake_first_strat, my_strat];

    // there is the possibility of some sick meta programming here, but ill park that for now
    for strat_function in strats_functions {
        let counts = Arc::new(Mutex::new(vec![]));
        let mut threads = vec![];
        for _ in 0..iter_count {
            let counts = Arc::clone(&counts);
            let words = CONTENTS.lines().collect(); // I don't really like this as it will reload the file over and over again
            threads.insert(0, thread::spawn(move || {
                counts.lock().unwrap().insert(0, strat_function(words));
            }))
        };
        for thread in threads {
            thread.join().unwrap()
        }

        let final_counts = (*counts.lock().unwrap()).clone();
        let sum: i32 = final_counts.clone().into_iter().sum(); // TODO shouldn't have to clone this
        let average = sum as f64 / final_counts.len() as f64;
        println!("Average guess count: {}", average);
        println!("------------------------------------------------------------");
    }
}