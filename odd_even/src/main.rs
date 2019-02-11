fn main() {
    let s = {
        let mut s = String::new();
        std::io::stdin().read_line(&mut s).unwrap();
        s.trim_right().to_owned()
    };
    let (a, b) = {
        let mut ws = s.split_whitespace();
        let a: i32 = ws.next().unwrap().parse().unwrap();
        let b: i32 = ws.next().unwrap().parse().unwrap();
        (a, b)
    };
//    let a = 3;
//    let b = 4;


    let ans = odd_even(&(a * b));
//    println!("answer is {}", ans);
    println!("{}", ans);
}

fn odd_even(x: &i32) -> String {
    let ans;
    if x % 2 == 0 {
        ans = String::from("Even");
    } else {
        ans = String::from("Odd");
    }
    ans
}
