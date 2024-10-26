use core::fmt;
use std::{collections::HashMap, fmt::Display};


pub enum LetterStatus {
    LetterMatch,
    WordMatch,
    NoMatch,
    Unknown,
}

impl Display for LetterStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            LetterStatus::LetterMatch => write!(f, "LetterMatch"),
            LetterStatus::WordMatch => write!(f, "WordMatch"),
            LetterStatus::NoMatch => write!(f, "NoMatch"),
            LetterStatus::Unknown => write!(f, "Unknown"),
        }
    }
}

pub struct Letter {
    pub(crate) status: LetterStatus,
    pub(crate) known_position: Option<u32>, // I don't want to do any crazy enum typing quite yet, ill get it running first, then I can do types properly
    // i8, as I hope the compiler will figure out that we can stack the entire array nicely into a single mem block
    // TODO: chad shit only, look into if this is actually how it does everything once compiled(i can't read assembly though, so that is a far down the road todo)
    pub(crate) false_positions: Option<[i8; 4]>, // there are a max of 4 wrong positions for a known letter
}

pub struct GameState<'a> {
    pub(crate) target_word: &'a str,
    pub(crate) letter_matches: HashMap<char, Letter>
}

pub fn initalize_game_state<'a>(game_state: &'a mut GameState<'a>) -> &mut GameState<'a> {
    for i in 97u8..=122 {
        game_state.letter_matches.insert(i as char, Letter{
            status: LetterStatus::Unknown,
            known_position: None,
            false_positions: None,
        });
    };
    
    game_state
}

pub fn wordle_compare<'a>(game_state: &'a mut GameState<'a>, guess: &'a str) -> &'a mut GameState<'a> {
    let target_chars: Vec<char> = game_state.target_word.chars().collect();
    let guess_chars: Vec<char> = guess.chars().collect();
    for (i, letter) in guess_chars.iter().enumerate(){
        if letter == &target_chars[i] {
            *game_state.letter_matches.get_mut(letter).unwrap() = Letter{
                status: LetterStatus::LetterMatch,
                known_position: Some(i as u32),
                false_positions: None,
            };
        } else if target_chars.contains(letter) {
            let mut existing_false_positions = game_state.letter_matches.get(letter).unwrap().false_positions.unwrap_or([0,0,0,0]);
            for existing_false_position in existing_false_positions.iter_mut() {
                if *existing_false_position == i as i8 { // if the location is already known, skip
                    break
                }
                if *existing_false_position == 0 { // if the next location is unknown, set a known incorrect position
                    *existing_false_position = i as i8;
                }
            }
            *game_state.letter_matches.get_mut(letter).unwrap() = Letter{
                status: LetterStatus::WordMatch,
                known_position: None,
                false_positions: Some(existing_false_positions),
            };
        } else {
            *game_state.letter_matches.get_mut(letter).unwrap() = Letter{
                status: LetterStatus::NoMatch,
                known_position: None,
                false_positions: None,
            }
        };
    };

    game_state
}