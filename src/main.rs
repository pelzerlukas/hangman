#[macro_use]
extern crate text_io;

use rand::Rng;
use std::fs;
use std::process;

const MAX_TRIES: u8 = 6;
const HIDDEN_LETTER_SYMBOL: char = '_';

fn main() {
    print!("Welcome to hangman! Good luck guessing the word! \n");
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
            return new(String::from(word));
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

fn new(value: String) -> GuessWord {
    let mut revealed = String::from("");
    for char in value.chars() {
        if char == ' ' {
            revealed += " ";
        } else {
            revealed += HIDDEN_LETTER_SYMBOL.to_string().as_str();
        }
    }
    return GuessWord {
        value: value,
        revealed: revealed,
        tries: 0,
        guesses: Vec::new(),
    };
}

fn read_input() -> Option<char> {
    let guess: String = read!("{}\n");
    let sanitized = guess.trim();
    return sanitized.chars().next();
}

impl GuessWord {
    fn reveal_letter(&mut self, index: usize) {
        let char = &self.value.chars().nth(index).unwrap().to_string();
        self.revealed
            .replace_range(index..index + 1, String::from(char).as_str());
    }

    fn guess_letter(&mut self, input: char) {
        let normalized_input = input.clone().to_ascii_lowercase();
        let already_guessed = *&self.guesses.contains(&normalized_input);
        if already_guessed {
            println!("You already guessed that!");
            return;
        } else {
            let _ = &self.guesses.push(normalized_input);
        }
        let mut found = false;
        for index in 0..self.value.len() {
            let char_at_index = self.value.chars().nth(index).unwrap();
            let input_matches_char_at_index =
                &char_at_index.to_ascii_lowercase() == &input.to_ascii_lowercase();

            if input_matches_char_at_index {
                self.reveal_letter(index);
                found = true;
            }
        }

        if !found {
            self.tries += 1;
            if &self.tries < &MAX_TRIES {
                println!("Too bad! This was attempt {} / {MAX_TRIES}", &self.tries);
                return;
            } else {
                println!("GAME OVER! The word was: {}", &self.value);
                process::exit(0);
            }
        }

        let won = !self.revealed.contains(HIDDEN_LETTER_SYMBOL);
        if won {
            println!("You won!!");
            process::exit(0);
        }
    }
}

struct GuessWord {
    value: String,
    revealed: String,
    tries: u8,
    guesses: Vec<char>,
}
