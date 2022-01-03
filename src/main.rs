use std::io;
fn main() {
    println!("Welcome user, kindly input your name");
    let mut username = String::new();
    io::stdin().read_line(&mut username)
        .expect("Failed to read line");
    println!("Welcome {} Kindly guess a number", username);
    let mut guess = String::new();
    io::stdin().read_line(&mut guess)
        .expect("Failed to read line");
    println!("You guessed {}", guess);
}
