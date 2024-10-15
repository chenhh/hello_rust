fn main() {
    #[cfg(feature = "foo")]
    {
        println!("foo enabled");
        println!("foo enabled2");
    }
    #[cfg(feature = "bar")]
    {
        println!("bar enabled");
        println!("bar enabled2");
    }

    println!("hello");
}