#[macro_use]
extern crate text_io;

use crate::guess_word::GuessWord;
use rand::Rng;
use std::fs;
mod guess_word;

fn main() {
    print!("Welcome to hangman! Good luck guessing t    he word! \n");
    let mut word_to_guess = get_word();
    loop {
        println!("Yet revealed: {}", word_to_guess.revealed);
        print!("Character guess: ");
        let char = read_input();
        match char {
            Some(result) => word_to_guess.guess_letter(result),
            None => continue,
        }
    }
}

fn get_word() -> GuessWord {
    let file = fs::read_to_string("./words.txt");
    match file {
        Ok(words) => {
            let word = get_random_word_of_file(words);
            return GuessWord::new(String::from(word));
        }
        Err(_) => {
            panic!()
        }
    }
}

fn get_random_word_of_file(words: String) -> String {
    let word_list: Vec<&str> = words.split("\n").collect();
    let word_count = word_list.len();
    let random_index = rand::thread_rng().gen_range(0..word_count);
    let random_word = word_list.get(random_index);
    let word = match random_word {
        Some(word) => word,
        None => panic!(),
    };
    return String::from(word.clone());
}

fn read_input() -> Option<char> {
    let guess: String = read!("{}\n");
    let sanitized = guess.trim();
    return sanitized.chars().next();
}
