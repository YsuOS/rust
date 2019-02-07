fn main() {
    let a = 3;
    let b = 4;

    let mut ans: String = odd_even(a * b);
    println!("answer is {}", ans);
}

fn odd_even(x : u32) -> mut String {
    if x % 2 == 0 {
        "Even"
    } else {
        "Odd"
    }
}
