/*
    函数相关
    1. 函数定义
    2. 函数参数
    3. 函数返回值
    4. 语句和表达式的区别
*/


fn main() {
    let a: i32 = -1;
    let b: u32 = 65;
    let c: i32 = 5;
    other_fun();

    other_fun1(a, b);

    let result = other_fun2(a, c);
    println!("fun2 = {}", result);
    println!("fun2 = {}", other_fun2(a, c));
    let result = other_fun3(a, c);
    println!("fun2 = {}", result);
    println!("fun2 = {}", other_fun3(a, c));
    let result = other_fun4(a, c);
    println!("fun2 = {}", result);
    println!("fun2 = {}", other_fun4(a, c));


    //语句是执行一些指令，但是不返回值的指令
    //let result = a + b; //语句，不返回值
    //let x = (let y = 1); //error

    //表达式会计算一些值。
    let y ={
        let x = 1;
        //x + 1; //此时不能加分号,加了会error
        x + 1
    };//此时应加分号,
    println!("y = {}",y);
    println!("y = {}",y ={
        let x = 1;
        //x + 1; //此时不能加分号,加了会error
        x + 1
    })
}

//定义函数,无参数，无返回值
fn other_fun() {
    println!("This is a function");
}

//有参，无返回值
//在定义函数参数时，不能使用自动判断，应该指定参数类型。
fn other_fun1(a: i32, b: u32) {
    println!("a = {},b = {}", a, b);
}

//有参，有返回值 fn1
fn other_fun2(a: i32, b: i32) -> i32 {
    let result = a + b;
    return result;
}
//有参，有返回值 fn2
fn other_fun3(a: i32, b: i32) -> i32 {
    let result = a + b;
    //可以将表达式写在大括号的最后一行（不加;)这样也可以确定函数的返回值
    result//后面不能用分号
    //result;
}
//有参，有返回值 fn3

fn other_fun4(a: i32, b: i32) -> i32 {
    a + b
}
