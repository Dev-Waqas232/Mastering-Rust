fn main() {
    // *  Tuples

    // ? defining a tuple
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    // ? accessing elements from a tuple
    let (x, y, z) = tup;

    let five_hundred = tup.0;

    let six_point_four = tup.1;

    let one = tup.2;

    println!("The value of y is: {y}");

    // * Array
    // declaring array
    let a = [1, 2, 3, 4, 5];

    let a: [i32; 5] = [1, 2, 3, 4, 5]; // with type

    let a = [3; 5]; // with default value of all elements at first position and the length of array at second position
}
