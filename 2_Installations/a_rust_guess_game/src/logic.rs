pub enum Dificulty {
    Easy,
    Hard,
}

pub enum GuessResualt {
    TooLow,
    TooHigh,
    Correct,
}
pub fn compaire(secret: u32, guess: u32) -> GuessResualt {}


//
// fn secret_number() -> u32 {
//     // Y: in this function we are returning only and only u32 value for that value to return we are not even adding variable.
//     // G: In rust anythign which chanses its internel state must be mutable. RNG changes its internel state every time its called.
//     let mut rng = rand::rng();
//     rng.random_range(1..=1000)
// }
//
// fn guess_input() -> String {
//     println!("| Guess The Number |");
//     let mut guess = String::new();
//     io::stdin()
//         .read_line(&mut guess)
//         .expect("⛔Falure to read line !!!");
//     guess
// }
