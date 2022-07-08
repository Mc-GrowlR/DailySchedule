#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}
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
enum Coin1{
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),//关联数据
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

    let absent_number: Option<i32> = None;

    //match
    let c = Coin1::Quarter(UsState::Alaska);
    println!("{}",value_in_centos1(c));

    //
    let v =0u8;
    match v {
        1 => println!("one"),
        3 => println!("three"),
        5 => println!("five"),
        7 => println!("seven"),
        _ => (),//下划线通配符要放到最后
    }
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
fn value_in_centos1(coin1:Coin1)->u8 {
    match coin1 {
        Coin1::Penny =>{
            println!("Penny!");
            1
        },
        Coin1::Nickel=>5,
        Coin1::Dime=>10,
        Coin1::Quarter(state) =>{
            println!("State quater from {:?}!",state);
            25
        },
    }//match表达式的结果，就是函数的返回结果
}