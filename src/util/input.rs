use std::io;

pub fn get_input(prompt: &str) -> char {
    println!("{}", prompt);
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    input.trim().chars().next().expect("No input")
}
