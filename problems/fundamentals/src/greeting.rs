use std::io;

fn main() {
    let mut name = String::new();

    println!("Please Enter Your Name : ");

    io::stdin()
        .read_line(&mut name)
        .expect("Failed to read the line!");

    println!("Hello {}", name);
}
