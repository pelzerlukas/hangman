#[macro_use]
extern crate text_io;

use crate::guess_word::GuessWord;
use std::fs;
mod guess_word;

fn main() {
    println!("Welcome to hangman! Good luck guessing the word!");
    let word_from_api = get_word_from_api();
    let mut word = match word_from_api {
        Ok(word) => word,
        Err(_) => get_word_from_backup_file(),
    };
    loop {
        println!("Yet revealed: {}", word.revealed);
        print!("Character guess: ");
        let char = read_input();
        match char {
            Some(result) => word.guess_letter(result),
            None => continue,
        }
    }
}

fn get_word_from_api() -> Result<GuessWord, Box<dyn std::error::Error>> {
    let word: Vec<String> =
        reqwest::blocking::get("https://random-word-api.vercel.app/api?words=1")?.json()?;
    let word = word
        .into_iter()
        .next()
        .ok_or("Error fetching word from api.")?;
    Ok(GuessWord::new(word))
}

fn get_word_from_backup_file() -> GuessWord {
    let backup_words = fs::read_to_string("./words.txt").unwrap();
    let word = get_random_word_of_file(backup_words);
    GuessWord::new(word)
}

fn get_random_word_of_file(words: String) -> String {
    let word_list: Vec<&str> = words.split("\n").map(|line| line.trim()).collect();
    let word_count = word_list.len();
    let random_index = rand::random_range(0..word_count);
    let random_word = word_list.get(random_index);
    let word = match random_word {
        Some(word) => word,
        None => panic!(),
    };
    String::from(*word)
}

fn read_input() -> Option<char> {
    let guess: String = read!("{}\n");
    let sanitized = guess.trim();
    sanitized.chars().next()
}
