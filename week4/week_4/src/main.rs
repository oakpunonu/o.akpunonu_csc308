use std::io;

fn main() {
    let mut sentence = String::new();
    println!("Write a sentence");
    io::stdin().read_line(&mut sentence).expect("Failed to read");
    let slice = sentence.trim();
    let last_word = slice.split_whitespace().last().unwrap();
    println!("Your last word is {}", last_word);
}