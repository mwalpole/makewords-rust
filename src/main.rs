use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    let npick = rand::thread_rng().gen_range(1..101);
    let mut turn = 1;

    println!("Please try to guess a number: {}", npick);

    loop{        
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read guess");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
    
        println!("Guess {turn}: {}", guess);

        match guess.cmp(&npick) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
        turn += 1;
    }
}
