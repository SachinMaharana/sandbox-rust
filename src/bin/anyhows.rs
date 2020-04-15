// use anyhow::Result;

#[derive(Debug)]

enum IpAddrKind {
    V4(String),
    V6(String),
}

fn main() {
    let four = IpAddrKind::V4(String::from("127.0.0.1"));
    let six = IpAddrKind::V6(String::from("::1"));

    route(four);
    route(six);

    let m = Message::Write(String::from("Hello"));
    m.call();
    // let some_num = Some(5);
    // let some_str = Some("a string");

    // let absent_num: Option<i32> = None;
    let five = Some(5);
    let six = plus(five);
    let none = plus(None);
    assert_eq!(none.is_none(), true);
    println!("{:?} {:?}", six, none);

    match six {
        None => {
            println!("Nothing");
        }
        Some(val) => println!("{}", val),
    }

    if let Some(x) = six {
        println!("{}", x);
    }
}

fn route(ip: IpAddrKind) {
    println!("{:?}", ip);
}

enum Message {
    // Quit,
    // Move { x: i32, y: i32 },
    Write(String),
    // ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {}
}

fn plus(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}
