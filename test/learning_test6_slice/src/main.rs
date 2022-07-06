fn main() {
    let s = String::from("Hello world");
    let wordIndex = first_world(&s);

    println!("{}", wordIndex);

    let s1 = String::from("hello world");
    let s2 = String::from("hello world");
    //字符串切片
    let hello = &s1[..5];
    //let hello = &s1[0..5];
    let world = &s1[6..11];
    //let world = &s1[6..s1.len()];
    //let world = &s1[6..];
    //指向整个字符串的语法糖
    let whole = &s1[0..s1.len()];
    //or
    //let whole = &s1[..];
    println!("{},{}", hello, world);
    let word_index = first_world1(&s2);
    println!("{}", word_index);
    //改进函数之后就不能使用：因为不能同时存在可变的借用和不可变得借用
    //s1.clear();

    //改进ver2
    //演示
    //string类型
    let my_string = String::from("Hello world");
    let word_index = first_world2(&my_string[..]);

    //使用字符串字面值，此时 my_string_literal 的类型为 &str
    let my_string_literal = "hello world";
    let word_index = first_world2(my_string_literal);

    //其他类型切片
    let a = [1, 2, 3, 4, 5];
    let slice_a = &a[0..3];
    println!("{}",slice_a[0]);
}

//此类函数设计的缺陷：必须要保证取出索引位置的字符串的有效性。
fn first_world(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }
    s.len()
}
//改进函数ver.1
fn first_world1(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    &s[..]
}
//改进函数ver.2
fn first_world2(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    &s[..]
}
