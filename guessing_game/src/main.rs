use rand::Rng;
use std::cmp::Ordering;
use std::env::args;
use std::io;
use std::io::prelude::*;

fn main() {
    let args: Vec<_> = args().skip(1).collect();

    let max_num: i32 = args.get(0).unwrap_or(&String::from("100")).parse().unwrap();

    println!("max_num: {}", max_num);

    println!("===============================================================");
    println!(
        "********** Guess a number between 0 and {}! *********",
        max_num
    );
    println!("===============================================================");

    print!("Enter least number for guessing game: ");
    io::stdout().flush().ok().expect("Could not flush stdout");

    // let min: u32 = read_user_input2();
    let min = match read_user_input() {
        Ok(num) => num,
        Err(_) => 0,
    };

    print!("Enter maximum number for guessing game: ");
    io::stdout().flush().ok().expect("Could not flush stdout");

    let max: u32 = match read_user_input() {
        Ok(num) => num,
        Err(_) => 100,
    };

    if min > max {
        println!("Min cannot be greater than max!");
        std::process::exit(1);
    }
    println!(
        "Thank you. You will not be asked to guess a number between {} and {}",
        min, max
    );
    let secret_number = rand::thread_rng().gen_range(min, max);
    println!("{}", secret_number);

    // keep track of number of guess
    let mut guess_count = 0;

    loop {
        println!("Please input your guess.");

        let guess: u32 = match read_user_input() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => {
                println!("Too small!");
                guess_count += 1;
            }
            Ordering::Greater => {
                println!("Too big!");
                guess_count += 1;
            }
            Ordering::Equal => {
                println!("You win! after {} wrong guess.", guess_count);
                break;
            }
        }
    }
}

fn read_user_input() -> Result<u32, std::num::ParseIntError> {
    let mut s = String::new();

    io::stdin().read_line(&mut s).expect("Failed to read line");

    s.trim().parse()
}
