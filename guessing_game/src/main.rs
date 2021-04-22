use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {

    let secret_number = rand::thread_rng().gen_range(1, 101);

    loop {
        
        println!("Enter a guess:");

        let mut guess = String::new();

        io::stdin()
        .read_line(&mut guess)
        .expect("Failed reading line");

        //Sshadowing..
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue
        };
        
        println!("You guessed {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Equal => {
                println!("It is Equal.. Guess is {}, Secret number is {}", guess, secret_number);
                break;
            },
            Ordering::Less => println!("It is Less.. Guess is {}, secret number is {}", guess, secret_number),
            Ordering::Greater => println!("It is Greater.. Guess is {}, Secret number is {}", guess, secret_number),
        }
    }
}
