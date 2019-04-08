fn main() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let v: Vec<u32> = input.trim().split_whitespace()
        .map(|x| x.parse().unwrap()).collect();
    //println!("{}", v[0]+1);
    let mut ans = 0;
    for x in 1..v[0]+1 {
        if (x < 10) {
            if (x % 10 >= v[1] && x % 10 <= v[2]){
//                println!("{}", x);
                ans += x;
            }
        } else if (x < 100) {
            if ((x % 10 + (x / 10) % 10) >= v[1] && (x % 10 + (x / 10) % 10) <= v[2]){
//                println!("{}", x);
                ans += x;
            }
        } else if (x < 1000) {
            if ((x % 10 + (x / 10) % 10 + (x / 100) % 10) >= v[1] && (x % 10 + (x / 10) % 10 + (x /100) % 10) <= v[2]){
//                println!("{}", x);
                ans += x;
            }
        } else if (x < 10000) {
            if ((x % 10 + (x / 10) % 10 + (x / 100) % 10 + (x / 1000) % 10) >= v[1] && (x % 10 + (x / 10) % 10 + (x /100) % 10 + (x / 1000) % 10) <= v[2]){
//                println!("{}", x);
                ans += x;
            }
        }
    }
    println!("{}", ans);
}
