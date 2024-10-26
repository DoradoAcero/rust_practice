use std::{collections::HashMap, fs};

mod wordle_utils;

use wordle_utils::{initalize_game_state, wordle_compare, GameState, LetterStatus};

fn main() {
    let file_path = "words.txt";

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");
    
    let words: Vec<&str> = contents.lines().collect();

    let target_word = words[rand::random::<usize>() % words.len()];

    let mut game_state = GameState {
        target_word,
        letter_matches: HashMap::new(),
    };
    // TODO I hate this, but we are going to figure it out later
    let mut game_state = initalize_game_state(&mut game_state);

    println!("target: {}", target_word);
    let guess = words[rand::random::<usize>() % words.len()];
    println!("guess: {}", guess);

    let game_state = wordle_compare(&mut game_state, guess);

    for (chr, letter) in game_state.letter_matches.iter() {
        match letter.status {
            LetterStatus::Unknown => {
                continue;
            }
            _ => {
                println!("{}: {}", chr, letter.status)
            }
        }
    }
    
}
