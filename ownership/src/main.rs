fn main() {
    {
        let s = "hello";
        println!("{}", s);
    }
//    println!("{}", s); // s is out of scope 
    let mut s = String::from("hello");
    s.push_str(", world!");
    println!("{}", s);

    let s1 = s;
    let s2 = s1;
//    println!("{}", s1); // s1 has already moved
    println!("{}", s2);

    let s1 = String::from("hello");
    let s2 = s1.clone();
    println!("s1:{}, s2:{}", s1, s2);

    let x = 5;
    let y = x;
    println!("x:{}, y:{}", x, y);

    let s = String::from("hello");
    takes_ownership(s);

    let x = 5;
    makes_copy(x);

    let s1 = gives_ownership();

    let s2 = String::from("hello");
    let s3 = takes_and_gives_back(s2);
    println!("s1:{}, s3:{}", s1, s3);

    let s1 = String::from("hello");
    let (s2, len) = calculate_length(s1);

    println!("The length of '{}' is {}", s2, len);

    let s1 = String::from("hello");
    let len = calculate_length1(&s1);

    println!("The length of '{}' is {}", s1, len);

//    let s = String::from("hello");
//    change(&s); // s is just refered, can't borrow

    let mut s = String::from("hello");
    change1(&mut s);
    println!("{}", s);

    {
    let r1 = &mut s; // mutable borrow occurs one time
//    let r2 = &mut s; 
    println!("r1: {}", r1);
    }
    let r2 = &mut s; 
    println!("r2: {}", r2);

    let reference_to_nothing = no_dangle();
    println!("{}", reference_to_nothing);
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

fn makes_copy(some_integer: i32){
    println!("{}", some_integer);
}

fn gives_ownership() -> String {
    let some_string = String::from("hello");
    some_string
}

fn takes_and_gives_back(a_string: String) -> String {
    println!("s2:{}", a_string);
    a_string
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();
    (s, length)
}

fn calculate_length1(s: &String) -> usize {
    s.len()
}

//fn change(some_string: &String) { // some_string is just refered, can't borrow
//    some_string.push_str(", world");
//} 

fn change1(some_string: &mut String) {
    some_string.push_str(", world");
}

//fn dangle() -> &String {
//    let s = String::from("hello");
//
//    &s
//}

fn no_dangle() -> String {
    let s = String::from("hello");

    s
}
