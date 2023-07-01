#![allow(dead_code)]
#![allow(unused)]

#[macro_use]
extern crate text_io;

fn main() {
    let word_to_guess = get_word();
    loop {
        let guess: String = read!("{}\n");

        if word_to_guess.value.contains(&guess) {
            print!("Appears in word! \n")
        } else {
            print!("Not in the word! \n")
        }
    }
}

fn get_input() -> std::io::Result<()> {
    let stdin = std::io::read_to_string(std::io::stdin())?;
    println!("Stdin was:");
    println!("{stdin}");
    Ok(())
}

fn get_word() -> GuessWord {
    return GuessWord {
        value: String::from("Henker"),
        revealed: String::from(""),
        tries: 0
    };
}



struct GuessWord {
    value: String,
    revealed: String,
    tries: u8
}
