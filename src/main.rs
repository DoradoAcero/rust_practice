use std::fs;


fn main() {
    let file_path = "words.txt";

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");
    
    let words: Vec<&str> = contents.lines().collect();

    let target_word = words[rand::random::<usize>() % words.len()];

    
}
