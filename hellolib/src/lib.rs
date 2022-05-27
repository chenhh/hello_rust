// function前面必須要加pub才可被其它crate使用，否則只能在local crate使用
pub fn myhello(name: &str) -> String {
    inner_hello();
    format!("hello name: {}", name)
}

fn inner_hello() {
    println!("hello inner function");
}