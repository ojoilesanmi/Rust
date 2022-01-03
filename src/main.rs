use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Welcome user, kindly input your name: ");
    let mut username = String::new();
    io::stdin().read_line(&mut username)
        .expect("Failed to read line");
    println!("Welcome {} Kindly guess a number: ", username);
    let secret_number = rand::thread_rng().gen_range(1, 100);
    loop{
        let mut guess = String::new();
        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");
        //println!("You guessed {}", guess);
        let guess: u32 = match guess.trim().parse(){
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("This is lower than the secret_number"),
            Ordering::Greater => println!("This is greater than the secret_number"),
            Ordering::Equal => {
                println!("Correct! You win");
                break;
            },

        }

    }
}
