use rand::Rng;
use std::io;
use std::cmp::Ordering;

fn main() {
    let generated_number = rand::thread_rng().gen_range(1..101);
    let mut tries: u32 = 0;

    println!("{}", generated_number);

    println!("I generated a number, now guess what it is.\n");

    loop {
        tries += 1;

        println!("Input a number:");

        let mut guessed_number = String::new();
        io::stdin()
            .read_line(&mut guessed_number)
            .expect("failed to read line.");

        let guessed_number: u32 = match guessed_number.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match guessed_number.cmp(&generated_number) {
            Ordering::Less => println!("too small"),
            Ordering::Greater => println!("too big"),
            Ordering::Equal => {
                println!("you win!");
                println!("You made {} attempts.", tries);
                break;
            }
        }
    }
}
