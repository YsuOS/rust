const MAX_POINTS : u32 = 100_000;

fn main() {
    let mut x = 5;
    println!("The value of x is : {}", x);
    x = 6;
    println!("The value of x is : {}", x);
    println!("The value of MAX_POINTS is : {}", MAX_POINTS);

    x = 5;
    let x = x + 1;
    let x = x * 2;
    println!("The value of x is : {}", x);

    let space = "  ";
    println!("The value of space is : {}", space);
    let space = space.len();
    println!("The value of space is : {}", space);

    let a = [1,2,3,4,5];
    let mut index = a.len();
    println!("The value of index is : {}", index);
    index = index + 1;
    println!("The value of index is : {}", index);
    let index = index + 1;
    println!("The value of index is : {}", index);
}
