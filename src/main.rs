#[derive(Debug)]
enum IpKind {
    V4,
    V6,
}

#[derive(Debug)]
struct IpAddress {
    version: IpKind,
    address: String,
    domain: String,
}

#[derive(Debug)]
enum Message {
    Quit,
    Move {
        x: i32,
        y: i32,
    },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn start(&self) {
        match self {
            Message::Quit => { println!("Process has been stopped") }
            Message::Write(input) => { println!("Output is {input}") }
            Message::ChangeColor(r, g, b) => {
                println!("Color has been set to R={}, G={}, B={}", r, g, b)
            }
            Message::Move { x, y } => { println!("Pointer moved to position X={} and Y={}", x, y) }
        }
    }
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn main() {
    let google: IpAddress = IpAddress {
        address: String::from("127.0.0.7"),
        version: IpKind::V4,
        domain: String::from("google.com"),
    };
    route(google);

    let microsoft: IpAddress = IpAddress {
        address: String::from("168.12.14.16"),
        version: IpKind::V6,
        domain: String::from("microsoft.com"),
    };

    route(microsoft);

    let message_1: Message = Message::Write(String::from("Hello world"));
    let message_2: Message = Message::ChangeColor(125, 54, 64);
    let message_3: Message = Message::Move { x: 120, y: 220 };
    let message_4: Message = Message::Quit;
    message_1.start();
    message_2.start();
    message_3.start();
    message_4.start();

    let bonus: Option<i32> = Some(100);
    let amount: i32 = 100;
    // cant resolve the oparation coz bonus may be not preset
    // println!("Total amount is {}", amount + bonus);

    match bonus {
        Some(bonus) => {
            println!("Total amount is {}", amount + bonus);
        }
        None => {
            println!("Total amount is {amount}");
        }
    }

    show_amount(Coin::Dime);
    show_amount(Coin::Nickel);
    show_amount(Coin::Penny);
    show_amount(Coin::Quarter);

    let dice = 6;

    match dice {
        3 => {
            println!("You just get half bonus");
        }
        6 => {
            println!("Hurray!. you get a bonus roll");
        }
        _ => {
            println!("Next move");
        }
    }
}

fn route(ip: IpAddress) {
    println!(
        "Request are forwarding to {:?} with version {:?} and the domain is {:?}",
        ip.address,
        ip.version,
        ip.domain
    );
}

fn show_amount(coin: Coin) -> u8 {
    match coin {
        Coin::Dime => 1,
        Coin::Nickel => { 10 }
        Coin::Penny => 100,
        Coin::Quarter => {
            println!("This is the quater bonus");
            return 250;
        }
    }
}
