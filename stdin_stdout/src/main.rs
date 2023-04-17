use std::io;
use rand::Rng;
use std::cmp::Ordering;


fn main() {
    println!("Guess the number!");
    println!("Please input a number between 1 and 100!");

    let random_number = rand::thread_rng().gen_range(1..=100);
    
    loop {
    
        let mut user_input = String::new();

        io::stdin()
            .read_line(&mut user_input)
            .expect("Failed to read line");

        let user_input: u32 = match user_input.trim().parse(){
            Ok(num) => num,
            Err(_) => continue,
        };
        
        println!("You guessed: {user_input}");

        match user_input.cmp(&random_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}