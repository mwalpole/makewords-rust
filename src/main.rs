use std::env;
use std::io;
use std::cmp::Ordering;
use filters::filter::Filter;
use rand::Rng;

// Capture and process args
// Demonstrate filtering
// Demonstrate test

struct EqualTo {
    pub length: usize,
}

impl Filter<usize> for EqualTo {
    fn filter(&self, n: &usize) -> bool {
        self.length == *n
    }
}

fn parse_args(args: &[String]) -> (Vec<&str>, usize) {
    let words = &args[1];
    let words: Vec<&str> = words
        .split(",")
        .collect();
    
    let length = &args[2];
    let length: usize = match length.trim().parse() {
        Ok(num) => num,
        Err(_) => 0,
    };
        
    (words, length)
}

fn word_length(words: Vec<&str>, length: usize) {

    let n: usize = words.len();

    println!("Demo");
    println!("{} found: {:?}", n, words);

    let eq = EqualTo { length: length };
    let result: Vec<&str> = words
        .into_iter()
        .filter(|s| eq.filter(&s.len()))
        .collect();
    
    println!("{} of length {}", result.len(), length);
}

fn word_guess(words: Vec<&str>) {
    let n: usize = words.len();
    let npick = rand::thread_rng().gen_range(1..n);
    let answer = words[npick];
    println!("Please guess the word: {}", answer);

    let mut turn = 1;

    loop{        
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read guess");

        let guess: usize = match guess.trim().parse() {
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

fn main() {
    let args: Vec<String> = env::args().collect();

    let (words, length) = parse_args(&args);
    let zero: usize = 0;

    match length.cmp(&zero) {
        Ordering::Less => println!("Length must be positive integer or None"),
        Ordering::Equal => word_guess(words),
        Ordering::Greater => word_length(words, length)
    }
}
