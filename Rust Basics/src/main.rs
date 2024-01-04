mod data_types;
use data_types::data_types;

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

    // Typecasting
    let a = 24 as i32;
    let b = 12 as i64;

    let c = (a as i64) / b; // a is typecasted as i64
    println!("c is: {}", c);

    let number = {
        let x = 3;
        x + 1 //No semicolon(Kinda unintuitive), It returns the value to the variable
    };
    println!("Printing weird expression,{}", number);

    // Function calling
    let sum: i32 = add_numbers(1, 2);

    println!("Sum of the numbers from function is: {}", sum);

    data_types();
}

fn add_numbers(x: i32, y: i32) -> i32 {
    return x + y; // Return statement
                  // x+y // Another way to return (No semicolon)
}
