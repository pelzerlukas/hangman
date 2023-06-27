#![allow(dead_code)]
#![allow(unused)]

//use std::io::{self, BufRead};
use console::Term;

fn main() {
    let word_to_guess = get_word();
    let mut input = String::new();

    loop {
        // \0 als character
        input = String::from("");
        let term = Term::stdout();
        let res = Term::read_char(&term);
        match res {
            Ok(a) => {
                const RADIX: u32 = 10;
                let num: u32 = a.to_digit(RADIX).unwrap_or_default();
                print!("Ok {}", &num);
            }
            Err(_) => print!("Error"),
        }
        print!("---------");

        /*io::stdin().read_line(&mut input);

            if word_to_guess.value.contains(&input) {
                print!("{} appears in word!", &input)
            } else {
                print!("Not in the word ({})!", &input)
            }

        }
        */
    }
}

fn get_word() -> GuessWord {
    return GuessWord {
        value: String::from("Henker"),
        revealed: String::from("_";value.len()),
    };
}

struct GuessWord {
    value: String,
    revealed: String,
}
