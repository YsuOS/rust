fn main() {
    let s = String::from("hello world");

    let word = first_word(&s);

    println!("{}", word);
    //s.clear(); // Error! if we have an immutable reference to something, we cannnot also take a
    //mutable reference.
    let my_string_literal = "hello world";
    println!("{}", first_word(my_string_literal));
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

