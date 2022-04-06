
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32
}

// multiple impl is supported, we can implement each methods in a impl. 但是拆开的效果和合并在一起的效果是一样的
impl Rectangle {
    // methods
    fn area(&self) -> u32 {
        self.width * self.height
    }
    // methods with more parameters
    fn can_hold(&self, other: &Rectangle) -> bool {
        return self.height > other.height && self.width > other.width;
    }
    // associate functions, different from methods, they are associated with the struct.
    // but they are still functions, becase they do'nt have an instance of the struct to work
    // with. You have already used the String::from associated function. use :: to call it.
    // jave coder: like static methods in java 
    fn square(size: u32) -> Rectangle {
        Rectangle { width: size, height: size }
    }
}

fn main() {
    let sqaure = Rectangle {
        width: 10,
        height: 100
    };
    println!("area of {:?} is {}", sqaure, sqaure.area());
    println!("area of {:#?} is {}", sqaure, sqaure.area());

    let other: Rectangle = Rectangle { width: 9, height: 8 };
    println!("can hold check, {:?}.can_hold {:?}, result is {}", 
        sqaure, other, sqaure.can_hold(&other));

    let sqaure2 = Rectangle::square(100);
    println!("A squre: {:?}", sqaure2);
}
