use std::{collections::HashMap, fs};

mod wordle_utils;

use wordle_utils::{get_possible_words, initalize_game_state, wordle_compare, GameState, LetterStatus};


fn run_basic_strat() -> i32 {
    let file_path = "words.txt";

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");
    
    let words: Vec<&str> = contents.lines().collect();

    let target_word = words[rand::random::<usize>() % words.len()];

    let mut game_state = GameState {
        target_word,
        letter_matches: HashMap::new(),
    };
    
    initalize_game_state(&mut game_state);

    println!("target: {}", target_word);
    let mut possible_words = words;

    let mut count = 0;
    loop {
        count += 1;
        let guess = possible_words.remove(rand::random::<usize>() % possible_words.len());
        println!("guess: {}", guess);

        wordle_compare(&mut game_state, guess);

        if guess == target_word{
            println!("FOUND");
            break;
        }
        possible_words = get_possible_words(&game_state, possible_words);
    };
    
    count
}

fn main() {
    let mut counts = vec![];
    for _ in 0..100 {
        counts.insert(0, run_basic_strat());
    };

    let sum: i32 = counts.iter().sum();
    let average = sum as f64 / counts.len() as f64;
    println!("Average guess count: {}", average);
}