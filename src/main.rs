#![allow(dead_code)]
#![allow(unused)]

#[macro_use]
extern crate text_io;

use std::process;

const MAX_TRIES: u8 = 3;

fn main() {
    print!("Welcome to hangman! Good luck guessing the word! \n");
    let mut word_to_guess = get_word();
    loop {
        print!("Character guess: ");
        let char = read_input();
        match char {
            Some(result) => word_to_guess.guess_letter(result),
            None => break,
        }
    }
}

fn get_word() -> GuessWord {
    return GuessWord {
        value: String::from("Henker"),
        revealed: String::from(""),
        tries: 0,
    };
}

fn read_input() -> Option<char> {
    let guess: String = read!("{}\n");
    let sanitized = guess.trim();
    return sanitized.chars().next();
}

impl GuessWord {
    fn guess_letter(&mut self, input: char) {
        //for char in self.value.chars() {}
        if self.value.contains(input) {
            //todo: ignorecase
            println!("Correct!");
        } else {
            self.tries += 1;
            if &self.tries < &MAX_TRIES {
                println!("Too bad! This was attempt {} / {MAX_TRIES}", &self.tries)
            } else {
                println!("GAME OVER! The word was: {}", &self.value);
                process::exit(0);
            }
        }
    }
}

struct GuessWord {
    value: String,
    revealed: String,
    tries: u8,
}
