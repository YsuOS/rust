fn main() {
    let a = 3;
    println!("{}", a);

//    let mut s = String::new();
//    std::io::stdin().read_line(&mut s).unwrap();
//    println!("{}", s);
    let mut s1 = String::new();
    s1 = "world".to_string();
    let i = s1.parse::<i32>().unwrap();
    println!("{:?}", i);
}
