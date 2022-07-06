struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool, //最后一行也得用 ,
}
#[derive(Debug)]
struct Rectangle {
    width: u32,
    length: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        //struct方法 中self参数必须小写
        self.width * self.length
    }
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width>other.width && self.length>other.length
    }
    //关联函数
    fn square(size:u32) ->Rectangle{
        Rectangle { width: size, length: size }
    }

}

fn main() {
    println!("Hello, world!");
    //实例化时，顺序不重要，但必须所有字段都得被赋值
    let user1 = User {
        email: String::from("acb@123.com"), //得使用 ，
        username: String::from("Nikky"),
        active: true,
        sign_in_count: 556,
    };
    //使用点标记法取得struct里面的某个值

    let rect = Rectangle {
        width: 30,
        length: 50,
    };
    let rect1 = Rectangle {
        width: 40,
        length: 50,
    };
    let rect2 = Rectangle {
        width: 20,
        length: 40,
    };
    let rect3 = Rectangle::square(55);
    println!("{}", rect.area());
    println!("{}",rect.can_hold(&rect1));
    println!("{}",rect.can_hold(&rect2));

    println!("{}", rect3.area());
}
