enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

fn main() {
    let v: Vec<i32> = Vec::new();
    let v = vec![1, 2, 3];

    let mut v = Vec::new();
    v.push(5);
    v.push(6);

    let third: &i32 = &v[0];
    let third: Option<&i32> = v.get(0);

    println!("{:?}", third);

//    let does_not_exist = &v[100];
//    let does_not_exist = v.get(100);
    
    let mut v = vec![1, 2, 3, 4, 5];
    let first = &v[0];

//    v.push(6);
    
    let v = vec![100, 32, 57];
    for i in &v {
        println!("{}", i);
    }

    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50;
        println!("{}", i);
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("bule")),
        SpreadsheetCell::Float(10.12),
    ];

    let data = "initial contents";
    let s = data.to_string();

    let s = "initial contents".to_string();
    let hello = String::from("こんにちは");
    println!("{}", hello);

    let mut s = String::from("foo");
    s.push_str("bar");
    let s2 = "bar";
    s.push_str(&s2);
    println!("{}", s2);

    let mut s = String::from("lo");
    s.push('l');
//    s3 = 'l';
//    s.push(&s3);
    println!("{}", s);

    let s1 = String::from("hello, ");
    let s2 = String::from("world");
    let s3 = s1 + &s2;
    println!("{}", s3);

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{}-{}-{}", s1, s2, s3);

    let s1 = String::from("hello");
//    let h = s1[0]; Rust strings don't support indexing!!
    let hello = "こんにちは";

    for c in hello.chars() {
        println!("{}", c);
    }
    for b in hello.bytes() {
        println!("{}", b);
    }

   use std::collections::HashMap;

   let mut scores = HashMap::new();

   scores.insert(String::from("Blue"), 10);
   scores.insert(String::from("Yellow"), 50);

   for (key, value) in &scores {
       println!("{}, {}", key, value);
   }
   scores.insert(String::from("Blue"), 25);
   println!("{:?}", scores);
   let teams = vec![String::from("Blue"), String::from("Yellow")];
   let initial_scores = vec![10, 50];

   let scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();

   let field_name = String::from("Blue");
   let field_value = String::from("Favorite color");

   let mut map = HashMap::new();
   map.insert(field_name, field_value);

   let mut scores = HashMap::new();
   scores.insert(String::from("Blue"), 10);

   scores.entry(String::from("Yellow")).or_insert(50);
   scores.entry(String::from("Blue")).or_insert(50);
   println!("{:?}", scores);

   let text = "hello world wonderful world";

   let mut map = HashMap::new();

   for word in text.split_whitespace() {
       let count = map.entry(word).or_insert(0);
       *count += 1;
   }

   println!("{:?}", map);
}
