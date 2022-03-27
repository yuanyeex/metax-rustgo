/**
 * use: to connect to other packages or dependencies. It's 'import' in java
 */
use std::io;
use std::cmp::Ordering;
use rand::Rng;

/**
 * fn: function declaration
 */
fn main() {
    /*
     * fucntion with '!' which means this is a macro 
     */
    println!("Guess the number!");
    // gen_range, incl21ude left but exclude right
    let secret_number:u32 = rand::thread_rng().gen_range(1, 101);
    // println!("The secret number is {}", secret_number);
    
    loop {
        println!("Please input your guess.");
        // mut makes guess mutable. In Rust, default un-modifiable
        // mut which is abbr for word mutable
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("Failed to read line");
        // let guess: i32 = guess.trim().parse().expect("Please type a numer!");
        // Using match and continue when input is invalid.
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input..., please input your guess in number:");
                continue;
            }
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too Large!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
        // &mut guess: 传入的是引用
    }
}
