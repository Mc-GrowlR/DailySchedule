fn main() {
    //mut 表述 可变变量
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 5;
    println!("The value of x is: {}", x);
    //使用下划线开头忽略未使用的变量
    //未使用下划线标注的变量会被编译器警告
    let _b = 6;

    //bool 类型,bool类型的值打印出来，是true 和false
    let is_true: bool = true;
    println!("is_true = {}", is_true);
    let is_false: bool = false;
    println!("is_false = {}", is_false);

    //char类型，在rust中char类型是32位的，原生支持UTF-8
    let is_char: char = 'a';
    println!("a = {}", is_char);
    let is_char: char = '我';
    println!("is_char = {}", is_char);

    //i8 , i16, i32, i64 ,u8, u16, u32, u63, f32, f64
    let is_int:i8 = -25;
    println!("is_int = {}",is_int);

    let is_float:f32 = 3.3;
    println!("isfloat = {}",is_float);

    //自适应类型，与平台有关：isize ,usize
    println!("max = {}",usize::max_value());

    //数组
    //定义方式：[Type;size]  ,size也是数组类型的一部分,在定义函数参数时，应当注意数组大小
    let arr:[u32;5] = [1,2,3,4,5];
    println!("arr[0] = {}",arr[0]);//下标从0开始
    show(arr);

    //元组
    //定义方式1 ：手动定义，确定数据类型
    let tup:(i32,f32 ,char)=(-3,3.69,'a');
    //定义方式2 ：自动推断
    let tup = (-3,3.69,'a');
    println!("tup = {}",tup.0);
    println!("tup = {}",tup.1);
    println!("tup = {}",tup.2);

    //依据已有元组茶创建变量。对元组进行拆解
    let (x,y,z)=tup;
    println!("{}",x);
    println!("{}",y);
    println!("{}",z);
}

fn show(arr:[u32;5]){
    println!("-------------------");
    for i in &arr{
        println!("{}",i);
    }
    println!("-------------------");

}

