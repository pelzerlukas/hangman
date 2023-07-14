use std::process;

const MAX_TRIES: u8 = 6;
pub const HIDDEN_LETTER_SYMBOL: char = '_';

impl GuessWord {
    pub fn new(value: String) -> GuessWord {
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
    fn reveal_letter(&mut self, index: usize) {
        let char = &self.value.chars().nth(index).unwrap().to_string();
        self.revealed
            .replace_range(index..index + 1, String::from(char).as_str());
    }

    pub fn guess_letter(&mut self, input: char) {
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

pub struct GuessWord {
    value: String,
    pub revealed: String,
    tries: u8,
    guesses: Vec<char>,
}
