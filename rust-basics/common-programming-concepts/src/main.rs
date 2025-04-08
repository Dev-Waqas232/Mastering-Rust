use std::io;

fn main() {
    // Shadowing
    let x = 5;

    println!("{}", x); // 5

    let x = x + 1;
    println!("{}", x); // 6

    {
        let x = x * 2;
        println!("{}", x); // 12
    }

    println!("{}", x); // 6

    // Program for invalid array element access
    let a = [1, 2, 3, 4, 5];

    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];

    println!("The value of the element at index {index} is: {element}");
}
