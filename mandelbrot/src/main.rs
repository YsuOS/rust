fn square_loop(mut x: f64) -> f64{
    for _i in 0..10{
        x = x * x;
    }
    x
}
fn main() {
    println!("square_loop 0.9 : {}", square_loop(0.9));
    println!("square_loop -0.9: {}", square_loop(-0.9));
    println!("square_loop 1.1 : {}", square_loop(1.1));
    println!("square_loop -1.1: {}", square_loop(-1.1));
}
