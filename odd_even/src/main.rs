fn main() {
    let a = 3;
    let b = 4;

    let ans = odd_even(&(a * b));
    println!("answer is {}", ans);
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
