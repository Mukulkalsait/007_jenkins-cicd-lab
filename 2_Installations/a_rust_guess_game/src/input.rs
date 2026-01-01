use std::io;

pub fn read_dificulty() -> String {
    let mut input: String = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("⛔ Failure to read.");
    input.trim().to_lowercase()
}

pub fn read_new_guess() -> u32 {
    let mut input: String = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("⛔ Failure to read.");
    let n: u32 = input.trim().parse::<u32>().unwrap_or_default();
    n
}
