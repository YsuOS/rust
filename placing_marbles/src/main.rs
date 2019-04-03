fn main() {
//    let s = {
//        let mut s = String::new();
//        std::io::stdin().read_line(&mut s).unwrap();
//        s.trim_right().to_owned()
//    };
//    let (a, b) = {
//        let mut ws = s.split_whitespace();
//        let a: i32 = ws.next().unwrap().parse().unwrap();
//        let b: i32 = ws.next().unwrap().parse().unwrap();
//        (a, b)
//    };
//    let a = 3;
//    let b = 4;
    let s = String::new();
    let s = "101".to_string();

    let ans = 0;
    if &s[0] == "1" {
        ans = ans + 1;
    }
    if &s[1] == "1" {
        ans = ans + 1;
    }
    if &s[2] == "1" {
        ans = ans + 1;
    }
    println!("{}", ans);
//    let ans = odd_even(&(a * b));
//    println!("answer is {}", ans);
//    println!("{}", ans);
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
