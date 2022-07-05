fn takes_ownership(some_string:String){
    println!("{}",some_string);
}

fn takes_ownership1(some_string:String)->String{
    println!("{}",some_string);
    some_string
}

fn makes_copy(i:i32){
    println!("i = {}",i);
}
fn main() {
    let s =String::from("hello");
    takes_ownership(s);
    //println!("{}",s);             //回报错，因为s已被回收掉了
    let s1 =String::from("hello");
    //takes_ownership1(s1);
    //println!("{}",s1);            //调用具有返回值的函数，仍会出错,需要有一个变量承载返回值
    let s2 = takes_ownership1(s1);
    println!("{}",s2);

    let x = 5;
    makes_copy(x);
    println!("{}",x);               //x能使用，因为x是被分配到栈上的，具有copy trait

    println!("Hello, world!");
}
