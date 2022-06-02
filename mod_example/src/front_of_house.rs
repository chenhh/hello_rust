// 直接定義hosting模組，
// 也可以pub mod hosting; 再將hosting.rs放在下一層模組
pub mod hosting {
    pub fn add_to_waitlist() {
        println!("hello waitlist");
    }
}