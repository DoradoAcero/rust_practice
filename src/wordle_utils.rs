

fn wordle_compare(target: &str, guess: &str) -> (u32, u32) {
    let mut target_chars: Vec<char> = target.chars().collect();
    let mut guess_chars: Vec<char> = guess.chars().collect();
    let mut correct = 0;
    let mut misplaced = 0;
}