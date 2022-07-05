//1. rust通过所有权机制来管理内存，编译器在编译就会根据所有权规则对内存的使用进行检查
//
//2. 堆和栈
//存储在栈上的大小须固定，但不等于大小固定就存储在栈上。将数据推入栈中并不被认为是分配”
//编译的时候数据的类型大小不固定，则被分配到堆上
//
//3. 作用域
//
//4. String内存回收
//5. 移动
//6. clone
//7. 栈上数据拷贝
//8. 函数和作用域
fn main() {
    //作用域
    let x:i32 = 1;              //x的作用域是从16行到main函数的右花括号  
    {
        let y:i32 = 1;          //y的作用域是从18行到20行的花括号
        println!("x = {}",x);
        println!("y = {}",y);
    }
    //println!("y = {}",y);     //会报错

    {
        // s1存储在堆上
        let mut s1 = String::from("hello");
        s1.push_str(" word");               //追加字符串
        println!("s1 = {}",s1);             //String类型离开作用域的时候会调用drop方法，释放内存

        let s2 = s1;
        println!("s2 = {}",s2);
        //println!("s1 = {}",s1);           //报错,因为move to s2 ,s1 invalid
        //s1不能再用了，因为已把所有权交给s2了 

        //clone() 方法：深拷贝
        let s3 = s2.clone();
        println!("s3 = {}",s3);
        println!("s2 = {}",s2);
    }

    //copy trait
    let a = 1;
    let b = a;
    println!("a = {} , b = {} ",a,b);
    //常用的具有copy trait 有：
    //所有整形
    //浮点型
    //布尔值
    //字符类型 char
    //元组
    
    println!("Hello, world!");
}
