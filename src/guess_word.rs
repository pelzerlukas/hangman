use crate::GameState;

const MAX_TRIES: u8 = 6;
pub const HIDDEN_LETTER_SYMBOL: char = '_';
pub const SPACE: char = ' ';

impl GuessWord {
    pub fn new(value: String) -> GuessWord {
        let revealed = create_revealed_field(&value);
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

    pub fn guess_letter(&mut self, input: char) -> GameState {
        let normalized_input = normalize_input(input);
        let already_guessed = self.guesses.contains(&normalized_input);
        if already_guessed {
            println!("You already guessed that!");
            return GameState::RUNNING;
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

        if !found {
            return apply_failed_attempt(self);
        }

        let won = !self.revealed.contains(HIDDEN_LETTER_SYMBOL);
        if won {
            println!("You won!! The word was: {}", self.value);
            return GameState::DONE;
        }

        GameState::RUNNING
    }
}

fn apply_failed_attempt(guess_word: &mut GuessWord) -> GameState {
    guess_word.tries += 1;
    if guess_word.tries < MAX_TRIES {
        println!(
            "Too bad! This was attempt {} / {MAX_TRIES}",
            &guess_word.tries
        );
        GameState::RUNNING
    } else {
        println!("GAME OVER! The word was: {}", &guess_word.value);
        GameState::DONE
    }
}

fn create_revealed_field(guess_word: &str) -> String {
    let mut revealed = String::from("");
    for char in guess_word.chars() {
        if char == SPACE {
            revealed.push(SPACE);
        } else {
            revealed.push(HIDDEN_LETTER_SYMBOL);
        }
    }
    revealed
}

pub struct GuessWord {
    value: String,
    pub revealed: String,
    tries: u8,
    guesses: Vec<String>,
}

fn normalize_input(to_normalize: char) -> String {
    String::from(to_normalize).to_lowercase()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn revealed_string_gets_created_successfully() {
        let input = "WordWith17Letters";
        let output = create_revealed_field(input);
        let a = output
            .chars()
            .filter(|char| char == &HIDDEN_LETTER_SYMBOL)
            .count();
        assert_eq!(a, input.len());
    }
}
