fn main() {

//    let s = String::from("101");
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();

//    println!("{}",s);
    let mut ans = 0;
    if &s[0..1] == "1" {
        ans = ans + 1;
    }
    if &s[1..2] == "1" {
        ans = ans + 1;
    }
    if &s[2..3] == "1" {
        ans = ans + 1;
    }
    println!("{}", ans);
}
