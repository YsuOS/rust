#[derive(Debug)]

enum IpAddrKind {
    V4,
    V6,
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

fn main() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;
    let msg_mv = Message::Move { x: 2, y: 3 };
    println!("{:?}", four);
    println!("{:?}", six);
    println!("{:?}", msg_mv);
}
