use std::collections::HashMap;

use wordle_utils::{get_possible_words, initalize_game_state, wordle_compare, GameState};

use crate::wordle_utils::{self, get_next_word};


pub fn run_basic_strat(words: Vec<&str>) -> i32 {
    let target_word = words[rand::random::<usize>() % words.len()];

    let mut game_state = GameState {
        target_word,
        letter_matches: HashMap::new(),
    };
    
    initalize_game_state(&mut game_state);

    // println!("target: {}", target_word);
    let mut possible_words = words;

    let mut count = 0;
    loop {
        count += 1;
        let guess = possible_words.remove(rand::random::<usize>() % possible_words.len());
        // println!("guess: {}", guess);

        wordle_compare(&mut game_state, guess);

        if guess == target_word{
            // println!("FOUND");
            break;
        }
        possible_words = get_possible_words(&game_state, possible_words);
    };
    
    count
}

// I just want to figure out how much difference smarter word selection at the start makes
pub fn stake_first_strat(words: Vec<&str>) -> i32 {
    let target_word = words[rand::random::<usize>() % words.len()];

    let mut game_state = GameState {
        target_word,
        letter_matches: HashMap::new(),
    };
    
    initalize_game_state(&mut game_state);

    // println!("target: {}", target_word);
    wordle_compare(&mut game_state, "rakes");
    wordle_compare(&mut game_state, "count");
    
    let mut possible_words = words;
    possible_words = get_possible_words(&game_state, possible_words);

    let mut count = 2;
    loop {
        count += 1;
        let guess = possible_words.remove(rand::random::<usize>() % possible_words.len());
        // println!("guess: {}", guess);

        wordle_compare(&mut game_state, guess);

        if guess == target_word{
            // println!("FOUND");
            break;
        }
        possible_words = get_possible_words(&game_state, possible_words);
    };
    
    count
}

pub fn my_strat(words: Vec<&str>) -> i32 {
    let possible_threshold = 7;
    let target_word = words[rand::random::<usize>() % words.len()];

    // println!("{}", target_word);

    let mut game_state = GameState {
        target_word,
        letter_matches: HashMap::new(),
    };
    
    initalize_game_state(&mut game_state);

    let mut possible_words = words.clone();

    let mut count = 0;
    loop {
        possible_words = get_possible_words(&game_state, possible_words);
        count += 1;

        // I love rust
        let guess = if possible_words.len() > possible_threshold {
            get_next_word(&mut game_state, &words)
        } else {
            // there is only one option, must be the correct word
            // println!("guess: {} target: {}", possible_words[0], target_word);
            possible_words.remove(rand::random::<usize>() % possible_words.len())
        };
        
        // println!("guess: {}", guess);
        // remove the guess from the possible words
        let index = possible_words.iter().position(|x| *x == guess);
        match index {
            Some(i) => {possible_words.remove(i);},
            None => ()
        };

        wordle_compare(&mut game_state, guess);

        if guess == target_word{
            // println!("FOUND");
            break;
        }
    };
    
    count
}