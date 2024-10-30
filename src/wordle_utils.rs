use std::{cmp, collections::HashMap, fmt::Display, hash::Hash};


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
    pub(crate) false_positions: Option<[u8; 4]>, // there are a max of 4 wrong positions for a known letter
}

pub struct GameState<'a> {
    pub(crate) target_word: &'a str,
    pub(crate) letter_matches: HashMap<char, Letter>
}

pub fn initalize_game_state(game_state: &mut GameState) {
    for i in 97u8..=122 {
        game_state.letter_matches.insert(i as char, Letter{
            status: LetterStatus::Unknown,
            known_position: None,
            false_positions: None,
        });
    };
}

pub fn wordle_compare(game_state: & mut GameState, guess: & str) {
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
            // TODO: handle case where LetterStatus is LetterMatch
            let mut existing_false_positions = game_state.letter_matches.get(letter).unwrap().false_positions.unwrap_or([0,0,0,0]);
            for existing_false_position in existing_false_positions.iter_mut() {
                if *existing_false_position == i as u8 { // if the location is already known, skip
                    break
                }
                if *existing_false_position == 0 { // if the next location is unknown, set a known incorrect position
                    *existing_false_position = i as u8;
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
}

// continue if it fails
macro_rules! unwrap_or_continue {
    ( $e:expr ) => {
        match $e.ok_or("null") {
            Ok(x) => x,
            Err(_) => {
                println!("continuing because null");
                continue;
            }
        }
    };
}

pub fn get_possible_words<'a>(game_state: &GameState, possible_words: Vec<&'a str>) -> Vec<&'a str> {
    let mut return_words  = vec![];
    let mut must_haves = vec![];

    for i in 97u8..=122{
        let chr = i as char;
        let letter = unwrap_or_continue!(game_state.letter_matches.get(&chr));
        match letter.status {
            LetterStatus::LetterMatch | LetterStatus::WordMatch => {
                must_haves.insert(0, (letter, chr));
            }
            _ => ()
        }
    }

    // do inital screening of 
    for word in possible_words.iter() {
        let mut flag = false;
        for (i, chr) in word.chars().enumerate() {
            let letter = unwrap_or_continue!(game_state.letter_matches.get(&chr));
            match letter.status {
                LetterStatus::NoMatch => {flag = true; continue},
                LetterStatus::WordMatch => {
                    // if the letter is in a wrong position, move on
                    for &num in &letter.false_positions.unwrap() {
                        if num == i as u8 {
                            flag = true;
                            continue;
                        }
                    }
                    if flag {continue;};
                },
                _ => continue
            }
        };
        if flag {continue}
        else {
            return_words.insert(0, *word);
        }
    }

    let mut to_remove = vec![];
    for (letter, chr) in must_haves {
        for word in &return_words {
            match letter.status {
                LetterStatus::LetterMatch => {
                    if !(word.chars().nth(letter.known_position.unwrap() as usize).unwrap() == chr) {
                        to_remove.insert(0, *word);
                    }
                },
                LetterStatus::WordMatch => {
                    if !word.contains(chr) {
                        to_remove.insert(0, *word);
                        continue;
                    }
                }
                _ => ()
            }
            
        }
    }
    for word in to_remove{
        return_words.retain(|&x| x != word);
    }

    return_words
}


pub fn get_next_word<'a>(game_state: &mut GameState, possible_words: &Vec<&'a str>) -> &'a str {
    // 1. Get the frequency of letters in the possible words
    let mut letter_freq = HashMap::new();
    for word in possible_words {
        for chr in word.chars() {
            match letter_freq.get_mut(&chr) {
                Some(freq) => *freq += 1,
                None => {letter_freq.insert(chr, 1); ()}
            }
        }
    };

    // 2. score the words based on the character frequency they have
    let mut word_scores = HashMap::new();
    for (chr, freq) in letter_freq.iter_mut() {
        match game_state.letter_matches.get(chr).unwrap().status {
            LetterStatus::Unknown => (),
            _ => *freq = 0
        }
    }
    
    for word in possible_words {
        word_scores.insert(word, 0);
        for chr in word.chars() {
            // sum the word based on letter freq
            *word_scores.get_mut(&word).unwrap() += letter_freq.get(&chr).unwrap_or(&0);
        }
    };

    *word_scores.iter().max_by(|a, b| a.1.cmp(b.1)).unwrap().0
}