enum IpAddrKind {
    V4(u8, u8, u8, u8),
    V6(String),
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(String),
}

fn main() {
    let four = IpAddrKind::V4(127, 0, 0, 1);
    let six = IpAddrKind::V6(String::from("::1"));
    
    print_ip(&four);
    print_ip(&six);
    
    let mut sum = 0;
    sum += value_in_cents(Coin::Penny);
    sum += value_in_cents(Coin::Nickel);
    sum += value_in_cents(Coin::Dime);
    sum += value_in_cents(Coin::Quarter(String::from("Alaska")));

    println!("sum of all coins: {sum}");

    let five = Some(5);
    let _six = plus_one(five);
    let _none = plus_one(None);
    
}

fn print_ip(ip: &IpAddrKind) {
    match ip {
        IpAddrKind::V4(a, b, c, d) => println!("IPv4 address: {}.{}.{}.{}"
        , a, b, c, d),
        IpAddrKind::V6(s) => println!("IPv6 address: {}", s),
    }
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("quarter from {state}");
            25
        },
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}
