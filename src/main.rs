use std::io;

fn main() {
    println!("Type in your word:");
    let mut word = String::new();
    io::stdin()
        .read_line(&mut word)
        .expect("Failed to read data");

    let mut length = calculate_length(&word);
    length -= 2;
    println!("Word {} have {} symbols", word, length);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}
