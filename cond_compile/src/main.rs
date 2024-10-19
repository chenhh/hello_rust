fn main() {
    #[cfg(feature = "cli")]
    {
        println!("cli enabled");
        println!("cli enabled2");
    }
    #[cfg(feature = "gui")]
    {
        println!("gui enabled");
        println!("gui enabled2");
    }

    println!("hello");
}