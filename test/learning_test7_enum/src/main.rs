enum IpAddrKind {
    V4,
    V6,
}

enum IpAddr {
    V4(String),
    V6(String),
}
enum IpAddr1 {
    V4(u8,u8,u8,u8),
    V6(String),
}

enum Coin{
    Penny,
    Nickel,
    Dime,
    Quarter,
}
fn main() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    //参数为枚举值的函数
    route(four);
    route(six);
    route(IpAddrKind::V6);
    println!("Hello, world!");

    let home = IpAddr1::V4(127,0,0,1);
    let loopback = IpAddr1::V6(String::from("::1"));

    //Option
    let some_number = Some(5);
    let some_string = Some("S String");

    let absent_number: OPtion<i32> = None;

    //match
}

fn route(ip_king: IpAddrKind){}
//match

fn value_in_centos(coin:Coin)->u8 {
    match coin {
        Coin::Penny =>1,
        Coin::Nickel=>5,
        Coin::Dime=>10,
        Coin::Quarter =>25,
    }//match表达式的结果，就是函数的返回结果
}