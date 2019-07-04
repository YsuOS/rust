extern crate num;
use num::Complex;

fn square_loop(mut x: f64) -> f64{
    for _i in 0..20{
        x = x * x;
    }
    x
}

fn square_add_loop(c: f64) -> f64{
    let mut x = 0.0;
    for _i in 0..1000{
        x = x * x + c;
    }
    x
}

#[allow(dead_code)]
fn complex_square_add_loop(c: Complex<f64>, limit: u32) {
    let mut z = Complex { re: 0.0, im: 0.0 };
    for i in 0..limit {
        z = z * z + c;
        if z.norm_sqr() > 4.0 {
            return Some(i);
        }
    }
    None
}
fn main() {
    println!("square_loop 0.9 : {}", square_loop(0.9));
    println!("square_loop -0.9: {}", square_loop(-0.9));
    println!("square_loop 1.1 : {}", square_loop(1.1));
    println!("square_loop -1.1: {}", square_loop(-1.1));

    let mut x = -0.3;
    while (x < 0.3) {
    println!("square_add_loop {} : {}", x, square_add_loop(x));
    x += 0.05;
    }
    println!("square_add_loop {} : {}", 0.251, square_add_loop(0.251));
    println!("square_add_loop {} : {}", 0.26, square_add_loop(0.26));

}
