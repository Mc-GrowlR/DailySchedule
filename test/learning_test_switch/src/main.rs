fn main() {

    //if
    let y = 1;
    if y == 1{
        println!("print : y = 1");
    }

    //if-else
    let y = 1;
    if y == 2{
        println!("print : y = 2");
    }
    else {
        println!("print : y != 2");
    }
    //if - else if -else
    if y == 3{
        println!("print : y = 3");
    }
    else if y == 1{
        println!("print : y = 1");
    }else {
        println!("print : y != 1");
    }

    //let中使用if
    let condition = true;
    let x = if condition {
        5       //不需要加分号，因为需要返回值
    } else {
        6       //不需要加分号，因为需要返回值
        //”six" //if - else 中的类型需要一致，因为x 的类型已在编译中确定
    };//此时是一个语句，需要加;
    println!("x = {}",x);

    //loop
    let mut counter = 0;
    loop {
        println!("in loop ");
        if counter == 10{
            break;
        }
        counter = counter +1;
        //counter +=1;
    }
    let result = loop {
        counter += 1;
        if counter == 20{
            break counter*2;
        }
    };//语句，需添加分号
    println!("result = {}",result);

    //while 
    let mut i =0;
    while i !=10{
        i+=1;
    }
    println!("i = {}",i);

    //for 
    println!("++++++for++++++");
    let arr:[u32;5] = [1,2,3,4,5];
    for element in arr.iter() {
        println!("element = {}",element);
    }
    
}
