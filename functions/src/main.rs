fn main() {
    // Hello, world!
    println!("Hello, world!");

    another_function(5, 6);

    let x = 8;
    let y = {
        let x = 3;
        println!("The value of x is : {}", x);
        plus_one(x)
    };

    another_function(x, y);
    let x : i32 = five();
    println!("The value of x is : {}", x);

}

fn another_function(x: u32, y: i32) {
    println!("Another function.");
    println!("The value of x is : {}", x);
    println!("The value of y is : {}", y);
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
