
// 定义一个模块module, 模块可以包含其他模块和函数.
// 使用分号，可以将module 分成多个文件
mod front_of_house;

// rust 的 path，可以使用绝对路径从一个crate 开始，或者使用相对路径从当前模块开始
pub  fn eat_at_restaurant() { 
    // 绝对路径
    crate::front_of_house::hosting::add_to_waitlist();

    // 相对路径
    front_of_house::hosting::add_to_waitlist();


}