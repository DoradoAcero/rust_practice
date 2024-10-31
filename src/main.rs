mod strats;
mod wordle_utils;

use std::{fs, sync::{Arc, LazyLock, Mutex}, thread, time::Instant};

use strats::{my_strat, run_basic_strat, stake_first_strat};


fn main() {
    let iter_count = 10000;
    static FILE_PATH: &str = "words.txt";

    static CONTENTS: LazyLock<String> = std::sync::LazyLock::new(|| {fs::read_to_string(FILE_PATH)
        .expect("Should have been able to read the file")});
    
    let strats_functions = vec![run_basic_strat, stake_first_strat, my_strat];

    // there is the possibility of some sick meta programming here, but ill park that for now
    for strat_function in strats_functions {
        let start = Instant::now();
        let counts = Arc::new(Mutex::new(vec![]));
        let mut handles = vec![];
        for _ in 0..iter_count {
            let counts = Arc::clone(&counts);
            let words = CONTENTS.lines().collect(); // I don't really like this as it will reload the file over and over again
            let handle = thread::spawn(move || { // this is how I see it in the docs, and how chatgpt did it, lets see if this changes anything, it shouldn't though
                
                // My theory, this waits for the function inside to finish, locking up all the other threads
                // Confirmed, I will leave this as a relic to myself as to how mutexes, threads and locking works

                // counts.lock().unwrap().insert(0, strat_function(words)); // SLOW

                let result = strat_function(words); // FAST
                counts.lock().unwrap().push(result); 
            });

            handles.push(handle);
        };
        for thread in handles {
            thread.join().expect("Thread failed to join");
        }

        let final_counts = (*counts.lock().unwrap()).clone();
        let sum: i32 = final_counts.clone().into_iter().sum(); // TODO shouldn't have to clone this
        let average = sum as f64 / final_counts.len() as f64;

        let speed = start.elapsed();
        println!("Average guess count: {}, Time Taken: {:.2?} {:.2?}/iter", average, speed, speed/iter_count);
        println!("------------------------------------------------------------");
    }
}