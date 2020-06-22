use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    let target = rand::thread_rng().gen_range(1, 101);
    let mut guesses_cnt: u32 = 0;
    
    // let foo = 0;
    // println!("foo is {}", foo);
    // println!("foo is {}", &foo);
    // println!("foo is {:p}", &foo);

    println!("Guess the number!");

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        guesses_cnt += 1;

        // Convert to a number (32 bit signed integer: i32)
        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&target) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win after {} {}!", guesses_cnt, if guesses_cnt > 1 {"guesses"} else {"lucky guess"});
                break;
            }
        }
    }
}
