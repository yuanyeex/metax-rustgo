
fn main() {
    println!("Hello, world!");
    let world = String::from("hello yuange");
    let ind = first_world(&world);
    println!("first world of {} at {}", world, ind);

    let first_word = first_world_slice(&world);
    println!("slice: {}", first_word);
}

fn first_world(string: &String) -> usize {
    let bytes = string.as_bytes();
    for (i, &elem) in bytes.iter().enumerate() {
        if elem == b' ' {
            return i;
        }
    }
    return string.len();
}

fn first_world_slice(string: &String) -> &str {
    let bytes = string.as_bytes();
    for (i, &elem) in bytes.iter().enumerate() {
        if elem == b' ' {
            return &string[..i];
        }
    }
    return &string[..];
}