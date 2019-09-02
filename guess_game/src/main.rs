use std::io;
use std::cmp::Ordering;
use rand::prelude::*;

fn main() {
    let number: i32 = rand::thread_rng().gen_range(1, 100);


    loop {
        let mut guess = String::new();
        println!("Guess random number");
        io::stdin().read_line(&mut guess).expect("Input Error");
        let guess_number: i32 = guess.trim().parse().expect("Not a number");
        match guess_number.cmp(&number) {
            Ordering::Less => println!("Bigger"),
            Ordering::Greater => println!("Lower"),
            Ordering::Equal => {
                println!("You Win!");
                break;
            },
        }
    }
}
