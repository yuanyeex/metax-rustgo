use std::io;
use rand::Rng;

fn main() {
    println!("Hello, world!");
    let mut buf: String = String::new();

    let sumup = sum(10, 320);
    println!("10 + 320 = {}", sumup);
    println!("20 + 320 = {}", sum(20, 320));

    block_express();

    println!("Input you name and press <ENTER>:");
    io::stdin().read_line(&mut buf).expect("read line from io failed");
    hello_world(buf);
    loop_while();
    loop_collection();
    // guess number again here, but we use if as the flow control instead of match.
    let secret = rand::thread_rng().gen_range(1, 100);
    guess_number(secret, 10);
}

fn hello_world(name: String) {
    println!("Hello world, {}", name);
}

fn sum(x: i32, y: i32) -> i32 {
    return x + y;
}

fn block_express() {
    // block is also an express which evaluate to a value
    let sum= {
        let a = 100;
        let b = 90;
        // cannot return and cannot add ';'
        a + b
    };

    println!("sum 100 and 90 in a block: {}", sum);
}

fn guess_number(secret_number: u32, limited_steps: u32) {
    println!("The secret is: {}", secret_number);
    
    for ind in 0..limited_steps {
        let mut num = String::new();
        println!("Input your guess number:");
        io::stdin().read_line(&mut num).expect("read line from io failed");
        let num:u32 = match  num.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid number input: {} ... ", num.trim());
                continue;
            }
        };

        if num == secret_number {
            println!("You win in {} steps!", ind + 1);
            break;
        } else if num < secret_number {
            println!("Too small!");
        } else {
            println!("Too larger");
        }
    }

    println!("Your steps run out!")
}

fn loop_while() {
    let mut num = 10;
    while num > 0 {
        println!("num: {}", num);
        // num = num - 1;
        num-=1;
    }
}

fn loop_collection() {
    let arr = [1, 2, 3, 4, 5, 6, 7];
    let mut ind = 0;
    while ind < arr.len() {
        println!("index at {} is {}", ind, arr[ind]);
        ind += 1;
    }

    for item in arr.iter() {
        println!("item looped: {}", item)
    }

    for ind in 0..arr.len() {
        println!("for loop, index {} holds value {}", ind, arr[ind])
    }
}