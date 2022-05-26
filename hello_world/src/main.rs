extern crate hellolib;

fn main() {
    let x = "hello world x";
    let y = x.to_string();
    println!("Hello, world! {}, {}", x, y);
    let stry: String =  y.chars().into_iter().take(8).collect();
    println!("{}, string len: {}", stry ,y.len());
    println!("calling hellolib name:{}", hellolib::myhello("chenhh"));
}
