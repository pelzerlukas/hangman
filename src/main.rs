#[macro_use]
extern crate text_io;

use crate::guess_word::GuessWord;
use std::fs;
mod guess_word;

struct Loop {
    guess_word: GuessWord,
    gamestate: GameState,
}

#[derive(PartialEq, Debug)]
enum GameState {
    RUNNING,
    DONE,
}

impl Loop {
    fn new(guess_word: GuessWord) -> Self {
        Self {
            guess_word,
            gamestate: GameState::RUNNING,
        }
    }

    fn next(&mut self, input: char) {
        if self.gamestate == GameState::RUNNING {
            let new_gamestate = self.guess_word.guess_letter(input);
            self.gamestate = new_gamestate;
        }
    }
}

fn main() {
    println!("Welcome to hangman! Good luck guessing the word!");
    let word_from_api = get_word_from_api();
    let word = match word_from_api {
        Ok(word) => word,
        Err(_) => get_word_from_backup_file(),
    };
    let mut game_loop = Loop::new(word);

    loop {
        println!("Yet revealed: {}", game_loop.guess_word.revealed);
        print!("Character guess: ");
        match read_input() {
            Some(char) => {
                game_loop.next(char);
                if game_loop.gamestate == GameState::DONE {
                    break;
                }
            }
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn succeed_with_only_correct_guesses() {
        let guess_word = GuessWord::new("Rust".to_string());
        let mut game_loop = Loop::new(guess_word);

        game_loop.next('r');
        assert_eq!(game_loop.guess_word.revealed, "R___");

        game_loop.next('t');
        assert_eq!(game_loop.guess_word.revealed, "R__t");

        game_loop.next('u');
        assert_eq!(game_loop.guess_word.revealed, "Ru_t");

        game_loop.next('s');
        assert_eq!(game_loop.guess_word.revealed, "Rust");
    }

    #[test]
    fn gamestate_gets_returned_correctly() {
        let guess_word = GuessWord::new("Rust".into());
        let wrong_letters = ['a', 'b', 'c', 'd', 'e', 'f'];
        let mut game_loop = Loop::new(guess_word);

        for wrong_letter_index in 0..wrong_letters.len() - 1 {
            game_loop.next(wrong_letters[wrong_letter_index]);
            assert_eq!(game_loop.gamestate, GameState::RUNNING);
        }

        let last_wrong_letter = *wrong_letters.last().unwrap();
        game_loop.next(last_wrong_letter);
        assert_eq!(game_loop.gamestate, GameState::DONE);
    }

    #[test]
    fn game_ends_after_being_in_done_once() {
        let guess_word = GuessWord::new("Rust".into());
        let wrong_letters = ['a', 'b', 'c', 'd', 'e', 'f'];
        let mut game_loop = Loop::new(guess_word);

        for wrong_letter_index in 0..wrong_letters.len() - 1 {
            game_loop.next(wrong_letters[wrong_letter_index]);
            assert_eq!(game_loop.gamestate, GameState::RUNNING);
        }

        let last_wrong_letter = *wrong_letters.last().unwrap();
        game_loop.next(last_wrong_letter);
        assert_eq!(game_loop.gamestate, GameState::DONE);

        let correct_letter = 'r';
        game_loop.next(correct_letter);
        assert_eq!(game_loop.gamestate, GameState::DONE);
    }
}
