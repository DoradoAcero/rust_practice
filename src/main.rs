use std::{collections::HashMap, fs, sync::{Arc, Mutex}, thread};
use std::time::Instant;

mod wordle_utils;

use wordle_utils::{get_possible_words, initalize_game_state, wordle_compare, GameState};


fn run_basic_strat(words: Vec<String>) -> i32 {
    let target_word = &words[rand::random::<usize>() % words.len()];

    let mut game_state = GameState {
        target_word,
        letter_matches: HashMap::new(),
    };
    
    initalize_game_state(&mut game_state);

    // println!("target: {}", target_word);
    let mut possible_words = words.clone(); // This should just be a bunch of pointers, it should be fine to clone

    let mut count = 0;
    loop {
        count += 1;
        let guess = &possible_words.remove(rand::random::<usize>() % possible_words.len());
        // println!("guess: {}", guess);

        wordle_compare(&mut game_state, guess);

        if guess == target_word{ // TODO look into if comparing references works here(with all this cloneing just to try parralellism, I don't really trust it)
            // println!("FOUND");
            break;
        }
        possible_words = get_possible_words(&game_state, possible_words);
    };
    
    count
}

fn main() {
    let now = Instant::now();
    let file_path = "words.txt";

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");
    
    let words: Vec<String> = contents.lines().map(|line| line.to_string()).collect();
    // Shared vector protected by a Mutex to ensure thread-safe access
    let counts = Arc::new(Mutex::new(Vec::new()));

    let mut handles = vec![];

    // Spawn 100 threads
    for _ in 0..1000 {
        let words_clone = words.clone();
        let counts_clone = Arc::clone(&counts); // Clone Arc to share the counts

        let handle = thread::spawn(move || {
            let result = run_basic_strat(words_clone);

            // Safely insert the result into the shared vector
            let mut counts = counts_clone.lock().unwrap();
            counts.push(result);
        });

        handles.push(handle);
    }

    // Wait for all threads to finish
    for handle in handles {
        handle.join().expect("Thread failed to join");
    }

    // Calculate the average from the collected results
    let counts = counts.lock().unwrap();
    let sum: i32 = counts.iter().sum();
    let average = sum as f64 / counts.len() as f64;

    println!("Average: {}", average);
    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
}