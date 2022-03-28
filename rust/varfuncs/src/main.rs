use std::io;
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