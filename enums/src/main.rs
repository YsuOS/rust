#[derive(Debug)] // this is attribute

enum IpAddrKind {
    V4,
    V6,
}
#[derive(Debug)]

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

//enum Option<T> {
//    Some(T),
//    None,
//}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}

fn value_in_cents(coin: Coin) -> u32 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn main() {
    let four = IpAddrKind::V4;
//    let six = IpAddrKind::V6;
    println!("{:?}", four);
//    println!("{:?}", six);

    let msg_mv = Message::Move { x: 3, y: 2 };
    println!("{:?}", msg_mv);

    let some_number = Some(5);
    let some_string = Some("a string");

    let absent_number: Option<i32> = None;

    let x: i8 = 5;
//    let y: Option<i8> = Some(5);

//    let sum = x + y;

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
    println!("{:?}", six);

    let quarter = value_in_cents(Coin::Quarter(UsState::Alabama));
    println!("{}", quarter);

//    let some_u8_value = 0u8;
//    let some_u8_value = 3;
//    match some_u8_value {
//        1 => println!("one"),
//        2 => println!("two"),
//        3 => println!("three"),
//        _ => (),
//    }
    
    let some_u8_value = Some(3);
    if let Some(3) = some_u8_value {
        println!("three");
    }
    let mut count = 0;
//    let coin = Coin::Quarter(UsState::Alabama);
    let coin = Coin::Dime;
    match coin {
        Coin::Quarter(state) => println!("State quarter from {:?}", state),
        _ => count += 1,
    }
    println!("{}", count)
}


