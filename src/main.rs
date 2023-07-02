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
        println!("Yet revealed: {}", word_to_guess.revealed);
        print!("Character guess: ");
        let char = read_input();
        match char {
            Some(result) => word_to_guess.guess_letter(result),
            None => break,
        }
    }
}

fn get_word() -> GuessWord {
    return new(String::from("Henker"));
}

fn new(value: String) -> GuessWord {
    let mut revealed = String::from("");
    for char in value.chars() {
        revealed += "_"
    }
    return GuessWord {
        value: value,
        revealed: revealed,
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
        if !self.value.contains(input) {
            self.tries += 1;
            if &self.tries < &MAX_TRIES {
                println!("Too bad! This was {} / {MAX_TRIES}", &self.tries);
                return;
            } else {
                println!("GAME OVER! The word was attempt: {}", &self.value);
                process::exit(0);
            }
        }

        for (index, char) in self.value.chars().enumerate() {
            if char == input {
                self.revealed
                    .replace_range(index..index + 1, String::from(char).as_str());
            }
        }

        if !self.revealed.contains('_') {
            println!("You won!!");
            process::exit(0);
        }
    }
}

struct GuessWord {
    value: String,
    revealed: String,
    tries: u8,
}
