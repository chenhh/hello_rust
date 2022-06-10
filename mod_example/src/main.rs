// 將 front_of_house 模組移動到屬於它自己的檔案 src/front_of_house.rs 中
mod front_of_house;

// 宣告使用mod
// pub use crate::front_of_house::hosting;
pub use front_of_house::hosting;

fn main() {
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
}
