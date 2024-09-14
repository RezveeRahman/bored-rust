/* We need this library to get user input */
use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    /*
     * The mut (mutable) keyword means the `guess` variable can change.
     * The `::new()` function creates an empty string.
     */
    let mut guess;
    let secret_number = rand::thread_rng().gen_range(0..=10);

    println!("Time to guess the number I'm thinking");

    println!("Now it's time to guess the number! It's definitely not {}.
              \r ;-)", secret_number);

    /* This a strange way rust does loop... */
    loop {
        /*
        * This line of code get's the standard in (stdin) from io. We are
        * essentially reading a line. `&` of guess denotes we are writing
        * or referencing the mutable guess variable. You need `&mut` for
        * this to work. The `expect` is for exceptions.    
        */
        guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("failed to read line");

        // convert string to integer for the comparison
        // let guess: u32 = guess.trim().parse().expect("Could not convert!");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);
        // println!("The correct number is {}", secret_number);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("The number is too Low!"),
            Ordering::Equal => {
                println!("You got it!");
                break;
            },
            Ordering::Greater => println!("High")
        }
    }
}
