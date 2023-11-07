use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let f = BufReader::new(File::open("words.txt").unwrap());
    let it = f.lines()
        .map(|line| line.unwrap())
        .map(|line| line.to_lowercase())
        .filter(|line| is_valid_word(line))
        .map(|line| line.split(":").next().unwrap().to_owned());
    for p in it {
        println!("{}", p);
    }
}

fn is_valid_word(word: &String) -> bool {
    let letters_for_today = "nowhetm";
    let center_letter = 'm';
    if word.len() <= 3 {
        return false
    }
    if !word.contains(center_letter) {
        return false
    }
    let invalid_characters = get_invalid_characters(letters_for_today);
    for c in invalid_characters.iter() {
        if word.contains(*c) {
            return false
        }
    }
    return true
}

fn remove_chars(input: &str, chars_to_remove: &str) -> String {
    let filtered_chars: String = input.chars()
        .filter(|c| !chars_to_remove.contains(*c))
        .collect();
    filtered_chars
}

fn get_invalid_characters(letters_for_today: &str) -> Vec<char>{
    let alphabet: &str = "abcdefghijklmnopqrstuvwxyz";
    let invalid_characters = remove_chars(alphabet,letters_for_today);
    let cvec: Vec<char> = invalid_characters.chars().collect();
    return cvec;
}