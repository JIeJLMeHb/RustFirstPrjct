use std::io;

fn main() {
    println!("Type in your username:");
    let mut username = String::new();
    io::stdin()
        .read_line(&mut username)
        .expect("Failed to read data");
    println!("Welcome, {}", username);
}
