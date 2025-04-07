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
}
