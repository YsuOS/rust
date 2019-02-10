use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
//    let a = 3;
//    let b = 4;

    let a: u32 = args[1].parse().unwrap();
    let b: u32 = args[2].parse().unwrap();

    let ans = odd_even(&(a * b));
//    println!("answer is {}", ans);
    println!("{}", ans);
}

fn odd_even(x: &u32) -> String {
    let ans;
    if x % 2 == 0 {
        ans = String::from("Even");
    } else {
        ans = String::from("Odd");
    }
    ans
}
