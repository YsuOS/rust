fn main() {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();
    s.trim();
    let mut t = String::new();

    let mut head: usize = 0;
    loop {
        if ( s[head..head+5].starts_with("erase") ) {
            t.push_str("erase");
            s.trim();
            println!("{}", s);
            if (s.len() == t.len() ) {
                println!("YES");
                return;
            } else {
                head += 5;
            }
        } else if ( s[head..head+5].starts_with("dream") ) {
            t.push_str("dream");
            if (s.len() == t.len() ) {
                println!("YES");
                return;
            } else {
                head += 5;
            }
        }
        if (s == t && s.len() == head - 1) {
            println!("YES");
            return;
        }
    }
    println!("NO");
}
