use std::process;

use crate::ascii_art::ASCII_ART;

const MAX_TRIES: usize = 6;
pub const HIDDEN_LETTER_SYMBOL: char = '_';

impl GuessWord {
    pub fn new(value: String) -> GuessWord {
        let mut revealed = String::from("");
        for char in value.chars() {
            if char == ' ' {
                revealed.push(' ');
            } else {
                revealed.push(HIDDEN_LETTER_SYMBOL);
            }
        }
        GuessWord {
            value,
            revealed,
            tries: 0,
            guesses: Vec::new(),
        }
    }
    fn reveal_letter(&mut self, char_index: usize, char: char) {
        let revealed = &mut self.revealed;
        let (byte_index, _) = revealed.char_indices().nth(char_index).unwrap();
        revealed.replace_range(
            byte_index..byte_index + HIDDEN_LETTER_SYMBOL.len_utf8(),
            String::from(char).as_str(),
        );
    }

    pub fn guess_letter(&mut self, input: char) {
        let normalized_input = normalize_input(input);
        let already_guessed = self.guesses.contains(&normalized_input);
        if already_guessed {
            println!("You already guessed that!");
            return;
        } else {
            let _ = &self.guesses.push(normalized_input.clone());
        }
        let mut found = false;
        for (index, char) in self.value.clone().chars().enumerate() {
            let input_matches_char_at_index = normalize_input(char) == normalized_input;

            if input_matches_char_at_index {
                self.reveal_letter(index, char);
                found = true;
            }
        }
        println!("{}", ASCII_ART[self.tries]);

        if !found {
            self.tries += 1;
            if self.tries < MAX_TRIES {
                println!("Too bad! This was attempt {} / {MAX_TRIES}", &self.tries);
                return;
            } else {
                println!("GAME OVER! The word was: {}", &self.value);
                process::exit(0);
            }
        }

        let won = !self.revealed.contains(HIDDEN_LETTER_SYMBOL);
        if won {
            println!("You won!! The word was: {}", self.value);
            process::exit(0);
        }
    }
}

pub struct GuessWord {
    value: String,
    pub revealed: String,
    tries: usize,
    guesses: Vec<String>,
}

fn normalize_input(to_normalize: char) -> String {
    String::from(to_normalize).to_lowercase()
}
