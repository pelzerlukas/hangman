#[macro_use]
extern crate text_io;

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
    return new(String::from("Bleistift"));
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
    };
}

fn read_input() -> Option<char> {
    let guess: String = read!("{}\n");
    let sanitized = guess.trim();
    return sanitized.chars().next();
}

impl GuessWord {
    fn guess_letter(&mut self, input: char) {
        let mut found = false;
        for (index, char) in self.value.chars().enumerate() {
            if char.to_lowercase().to_string() == input.to_lowercase().to_string() {
                self.revealed
                    .replace_range(index..index + 1, String::from(char).as_str());
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
}
