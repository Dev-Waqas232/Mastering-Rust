use std::io;

fn main() {
    let mut num = String::new();
    io::stdin()
        .read_line(&mut num)
        .expect("Failed to read line!");

    let num: u32 = match num.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid input!");
            return;
        }
    };

    if num % 2 == 0 {
        println!("Number is Even!")
    } else {
        println!("Number is Odd!")
    }
}
