fn main() {
    println!("Hello, world!");

    another_function(5, 6);
}

fn another_function(x: u32, y: i32) {
    println!("Another function.");
    println!("The value of x is : {}", x);
    println!("The value of y is : {}", y);
}
