use std::fmt::Debug;
use rand::Rng;
use rand::thread_rng;

#[derive(Debug)]
#[allow(dead_code)]
enum IpAddrKind {
    V4(String),
    V6(String),
}

// #[derive(Debug)]// enum is more concise...
// struct IpAddr {
//     kind: IpAddrKind,
//     address: String,
// }

// compare message enum vs structs

#[allow(dead_code)]
enum Message { // Single type that handles multiple  things,
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
} // enums allow for type flexibility..

impl Message {
    fn call(&self) {
        match self {
            Message::Quit => {
                println!("Its time to quit");
            },
            Message::ChangeColor(r, g, b)=> {
                println!("r={r}, g={g}, b={b}");
            },
            Message::Move {x, y}=> {
                println!("x={x}, y={y}");
            },
            Message::Write(text) => {
                println!("Writing value: {text}");
            }
        };
    }
}

///// would need ltos of structs instead; each have different type too!
// struct QuitMessage; // unit struct
// struct MoveMessage {
//     x: i32,
//     y: i32,
// }
// struct WriteMessage(String); // tuple struct
// struct ChangeColorMessage(i32, i32, i32); // tuple struct


#[derive(Debug)] // so we can inspect the state in a minute
#[allow(dead_code)]
enum UsState {
    Alabama,
    Alaska,
    Arizona,
    Arkansas,
    California,
    Colorado,
    Connecticut,
    Delaware,
    Florida,
    Georgia,
    Hawaii,
    Idaho,
    Illinois,
    Indiana,
    Iowa,
    Kansas,
    Kentucky,
    Louisiana,
    Maine,
    Maryland,
    Massachusetts,
    Michigan,
    Minnesota,
    Mississippi,
    Missouri,
    Montana,
    Nebraska,
    Nevada,
    NewHampshire,
    NewJersey,
    NewMexico,
    NewYork,
    NorthCarolina,
    NorthDakota,
    Ohio,
    Oklahoma,
    Oregon,
    Pennsylvania,
    RhodeIsland,
    SouthCarolina,
    SouthDakota,
    Tennessee,
    Texas,
    Utah,
    Vermont,
    Virginia,
    Washington,
    WestVirginia,
    Wisconsin,
    Wyoming,
}

#[allow(dead_code)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

impl Coin {
    fn value_in_cents(&self) -> u8 {
        match self {
            Coin::Penny => 1,
            Coin::Nickel => 5,
            Coin::Dime => 10,
            Coin::Quarter(state) => {
                println!("You got a special quarter from {:?}", state);
                25
            },
        }
    }
}



pub fn main() -> () {
    // Enums - https://rust-book.cs.brown.edu/ch06-01-defining-an-enum.html
    let home = IpAddrKind::V4(String::from("127.0.0.1"));
    let loopback = IpAddrKind::V6(String::from("::1"));

    println!("{:?}", home);
    println!("{:?}", loopback);
    //println!("{} {}", home.)

    //Message example
    //let msg = Message::Write("ThisIsCool".to_string());
    //let msg = Message::Quit;
    //let msg = Message::ChangeColor(0, 0, 0);
    let msg = Message::Move {x: 15, y: 25};
    msg.call();

    //Option enum

    let some_number = Some(5);
    let some_char = Some('e');

    let absent_number: Option<i32> = None;

    println!("{:?} {:?} {:?}", some_number, some_char, absent_number);

    let x: i8 = 5;
    let y: Option<i8> = None;//Some(3);
    println!("{}",x + y.unwrap_or(0)); // unwrap_or grabs Some(); or gives the default else

    // Alternatively use the match statement..
    let sum = match y {
        Some(y) =>  x+y,
        None => x
    };
    println!("{sum}");

    // 6.2 - match Control Flow - https://rust-book.cs.brown.edu/ch06-02-match.html

    println!("{}", Coin::Quarter(UsState::Mississippi).value_in_cents());



    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    println!("{} {} {}", five.unwrap_or(0), six.unwrap_or(0), none.unwrap_or(0));

    // dice roll  practice.

    //let dice_roll = 9;
    let dice_roll: i32 = thread_rng().gen_range(1..=12);
    println!("{dice_roll}");
    match dice_roll {
        3 =>  println!("add_fancy_hat()"),
        7 => println!("remove_fancy_hat()"),
        other => println!("Move this many spaces.. {}", other),
    }

    let opt: Option<String> =
        Some(String::from("Hello world"));

    // Matching on Reference so that grabbing s doesn't transfer ownership of opt... (which is not returned)
    match &opt { // match &opt does work here even with Some(s).
        Some(s) => println!("Some! {s}"), // this compiles if Some(_) but not Some(s)
        None => println!("None!")
    };

    println!("{:?}", opt);

    //6.3 - if / let => https://rust-book.cs.brown.edu/ch06-03-if-let.html

    let config_max = Some(3u8);

    // Here's a match script to handle options and print if it finds osmething..
    match config_max {
        Some(max) => println!("The maximum is configured to be {max}"),
        _ => (),
    }

    // Alternatively can use the "if let" syntax - defines "max"
    let config_max = Some(3u8);
    if let Some(max) = config_max {
        println!("The maximum is configured to be {max}");
    }

    // if let is syntactic sugar for match..else

    let mut count = 0;
    let coin = Coin::Quarter(UsState::Colorado);
    //let coin = Coin::Dime;
    // Match statement -
    match &coin {
        Coin::Quarter(state) => println!("State quarter from {state:?}!"),
        _ => count += 1,
    }

    // doesn't have to be a Some None (option)
    if let Coin::Quarter(state) = coin {
        println!("State quarter from {state:?}!");
    } else {
        count += 1;
    }

    println!("{count}");



}



fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}