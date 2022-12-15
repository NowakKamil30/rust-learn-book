use std::fmt::rt::v1::Count::Param;

enum Movement {
    Up,
    Down,
    Left,
    Right
}

enum IpAddrKind {
    V4(u8, u8, u8, u8),
    V6(String)
}

struct IpAddr {
    kind: IpAddrKind,
    address: String
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32, i32)
}

impl Message {
    fn say_hello() {
        println!("hello")
    }
}

fn move_avatar(m: Movement) {
    match m {
        Movement::Up => println!("Up"),
        Movement::Down => println!("Down"),
        Movement::Left => println!("Left"),
        Movement::Right => println!("Right")
    }
}

pub fn run() {
    move_avatar(Movement::Down);
    // using a struct and enum without value
    // let localhost = IpAddr {
    //     kind: IpAddrKind::V4,
    //     address: String::from("127.0.0.1")
    // };

    // enum with value
    let localhost = IpAddrKind::V4(127, 0 ,0 ,1);

    let localhost = match localhost {
        IpAddrKind::V4(first_part, second_part, third_part, fourth_part) =>
            format!("{}:{}:{}:{}", first_part, second_part, third_part, fourth_part),
        IpAddrKind::V6(ip_address) => ip_address
    };

    Message::say_hello();

    // Rust doesn't have null value, it has a optional enum
    let some_number = Some(5);
    let some_string = Some("aaaa");

    let none_example: Option<i32> = None; // we have to specify the type

    let x: u8 = 5;
    let y: Option<u8> = Some(5);

    let sum = x + y.unwrap_or(0);

    let six = plus_one(y);

    let some_value = Some(3);
    match some_value {
        Some(3) => println!("three"), // we want to do something only if value is 3
        _ => ()
    }

    if let Some(3) = some_value {
        println!("three");
    }
}

fn plus_one(x: Option<u8>) -> Option<u8> {
    match x {
        None => None,
        Some(i) => Some(i + 1)
    }
}