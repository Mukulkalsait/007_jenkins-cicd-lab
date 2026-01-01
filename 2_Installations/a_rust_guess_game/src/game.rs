use crate::input;
use crate::logic;
use crate::random;
use colored::*;

pub fn set_dificulty() {
    println!(
        "|==================> Please Select The Level {} vs {} <==================|",
        "'Easy'".green(),
        "'Hard'".red()
    );
}

pub fn start() {
    let secret = random::get_secret();
    loop {
        let guess = input::read_new_guess();
        match logic::compaire(secret, guess) {}
    }
}
