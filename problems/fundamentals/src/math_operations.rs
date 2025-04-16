use std::io;

fn main() {
    let mut num1 = String::new();
    let mut num2 = String::new();
    let mut op = String::new();

    println!("Please Enter First Number : ");

    io::stdin()
        .read_line(&mut num1)
        .expect("Failed to read the line");

    let num1: i32 = match num1.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Can't parse to integer");
            return;
        }
    };

    println!("Please Enter Second Number : ");

    io::stdin()
        .read_line(&mut num2)
        .expect("Failed to read the line");

    let num2: i32 = match num2.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Can't parse to integer");
            return;
        }
    };

    println!("Please Enter Operation : ");

    io::stdin()
        .read_line(&mut op)
        .expect("Failed to read the line");

    let ch = match op.trim().chars().next() {
        Some(c) => c,
        None => {
            println!("No character entered!");
            return;
        }
    };

    if ch == '+' {
        println!("Sum of {} and {} is {}", num1, num2, num1 + num2);
    } else if ch == '-' {
        println!("Difference of {} and {} is {}", num1, num2, num1 - num2);
    } else if ch == '*' {
        println!("Product of {} and {} is {}", num1, num2, num1 * num2);
    } else if ch == '/' {
        println!("Division of {} and {} is {}", num1, num2, num1 / num2);
    } else {
        print!("Invalid Operation")
    }
}
