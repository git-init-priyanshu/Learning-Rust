fn main() {
    let x = 1; //Implicitly assigning the variable
    let y: u32 = 2; //Explicitly assigning the variable
    let _y = 2 as u32; // Another way to explicitly assign data type
    let _y = 2u32; // Another way

    println!(" Value of x is : {}", x); // Print value -> Value of x is : 1
    println!(" Value of y is : {}", y);
    // y = 5; // This wil throw error, by default the variables are immutable

    let mut z: u32 = 2; // Making a mutable variable
    println!("z:{}", z);
    z = 8; // Assigning different value to the variable
    println!("z:{}", z);
    // z = "hello World!" // Cannot do this, Statically typed language

    let x = "hello"; // Variables can be redefined and even to a different data type
    println!(" Value of x is : {}", x);

    const CONSTANT_VARIABLE: u32 = 540; // constants cannot be redefined or reassigned
                                        // const CONSTANT_VARIABLE: u32 = 30; // cannot do this
    println!("constant value: {}", CONSTANT_VARIABLE);

    // Datatypes
    //Scalars (Stores one value)

    // Integers
    let _x: i32 = -1; // default value
    // Unsigned Integer
    let _x: u32 = 1;
    // float
    let _x: f32 = 10.9;
    let _x: f64 = 10.99; // default value
    // Boolean
    let _x: bool = false;
    // Character
    let _x: char = 'c';

    // Compounds (Stores multiple value)

    // Tuple
    let tup: (i32, bool, char) = (-2, false, '9'); // can be mutable with 'mut'
    println!("tuple 1st value: {}", tup.0);
    println!("tuple 2nd value: {}", tup.1);
    println!("tuple 3rd value: {}", tup.2);

    // Array
    let arr: [i32; 5] = [1, 2, 3, 4, 5]; // [i32,5] -> There will be 5 elements and all should be i32
    println!("Array value at index 0:{}", arr[0]);

    // Typecasting
    let a = 24 as i32;
    let b = 12 as i64;

    let c = (a as i64) / b; // a is typecasted as i64
    println!("c is: {}", c);
}
