mod front_of_house {
    mod hosting {
        fn add_to_waitlist() {}
    }
}
pub fn eat_at_restaurant() {
    
    //调用mod中函数的两种方法：
    //1. 绝对路径
    crate::front_of_house::hosting::add_to_waitlist();
    //2. 相对路径派生类不使用 `new`
    front_of_house::hosting::add_to_waitlist();
}