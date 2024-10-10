use std::fs::File;
use std::io::prelude::*;

fn create_file(file_name: &str) -> std::io::Result<()> {
    let mut file = File::create(file_name)?;
    file.write_all(b"Hello, world!")?;
    Ok(())
}

fn main() {
    if let Ok(_) = create_file("foo.txt"){
        println!("File created!");
    }
}
