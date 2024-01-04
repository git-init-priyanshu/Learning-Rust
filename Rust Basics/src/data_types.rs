use std::collections::HashMap;

pub fn data_types() {
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

    // Structs
    struct User {
        username: String,
        email: String,
        password: String,
        is_active: bool,
        sign_in_count: u32,
    }
    let mut user1: User = User {
        username: String::from("user1"),
        email: String::from("user1@email.com"),
        password: String::from("Pass@123"),
        is_active: false,
        sign_in_count: 1,
    };
    println!("username: {}", user1.username);
    println!("email: {}", user1.email);
    println!("password: {}", user1.password);
    println!("is_active: {}", user1.is_active);
    println!("sign_in_count: {}", user1.sign_in_count);

    user1.username = String::from("NewUserName");
    println!("New username: {}", user1.username);

    let user2 = User {
        username: String::from("user2"),
        email: String::from("user2@mail.com"),
        ..user1
    };

    println!("username: {}", user2.username);
    println!("email: {}", user2.email);
    println!("password: {}", user2.password);
    println!("is_active: {}", user2.is_active);
    println!("sign_in_count: {}", user2.sign_in_count);

    let rect: Rectangle = Rectangle {
        width: 30,
        height: 40,
    };
    println!("rect: {:#?}", rect); // {:#?}: this syntax allows printing
                                   // Using struct Implementation
    println!("Area of rectangle: {}", rect.area());

    // Enums
    enum Modes {
        Dark(String), // () is used to store data in the enum itself
        Light(String),
    }
    let _dark_mode = Modes::Dark(String::from("Dark")); // :: is used to specify varients of enum
    let _light_mode = Modes::Light(String::from("Light"));

    // Using enum Implementation
    println!("{:#?}", ResponseType::get_res_types());

    // Option Enum
    let x = 4;
    let y = Some(5);

    // Match expresssion (Exhaustive)
    let value_of_y = match y {
        None => 0,
        Some(i) => i,
    };
    let sum = x + y.unwrap_or(0); // Same as above expression

    println!("{} <-> {}", sum, value_of_y + x);

    // Vector (or dynamic array)
    let mut v: Vec<i32> = Vec::new(); //Initializing vector
                                      // Pushing elements to vector
    v.push(1);

    let v2 = vec![1, 2, 3]; // Initializing the vector with vec macro.
    let third_element = &v2[2];
    println!("Third element of v2: {}", third_element);

    // This method of getting elemet is error prone as we can specify out of bound index
    // We can use get method and match expression to deal with this
    match v2.get(2) {
        Some(third_element) => println!("Third element of v2: {}", third_element),
        None => println!("There is no third element"),
    }
    // Printing elements in vector
    for e in &v2 {
        println!("{}", e);
    }

    // Hashmaps
    let mut hashmap = HashMap::new();
    hashmap.insert(1, "hello");
    hashmap.insert(2, "world!");

    for (_key,value) in hashmap{
        println!("{}",value);
    }
    
}

// Struct Implementation
#[derive(Debug)] // deriving "Debug" trait -> Makes easier to print
struct Rectangle {
    width: u32,
    height: u32,
}
impl Rectangle {
    fn area(&self) -> u32 {
        return &self.width * &self.height;
    }
}

// Enum Implementation
#[derive(Debug)] // deriving "Debug" trait -> Makes easier to print
struct Response {
    success: bool,
    status_code: u16,
    msg: String,
}
#[derive(Debug)]
enum ResponseType {
    Error(Response),
    Success(Response),
}
impl ResponseType {
    fn get_res_types() -> ResponseType {
        let response: Response = Response {
            success: true,
            status_code: 200,
            msg: String::from("Good Response"),
        };
        if response.status_code == 200 {
            return ResponseType::Success(response);
        } else {
            return ResponseType::Success(response);
        }
    }
}
