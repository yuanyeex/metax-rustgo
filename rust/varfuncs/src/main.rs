use std::io;
fn main() {
    println!("Hello, world!");
    let mut buf: String = String::new();

    let sumup = sum(10, 320);
    println!("10 + 320 = {}", sumup);
    println!("20 + 320 = {}", sum(20, 320));

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