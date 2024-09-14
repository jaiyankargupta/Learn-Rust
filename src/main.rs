// fn main() {
//     println!("Hello, world!");
// }

// fn add(num1: i32, num2: i32) -> i32 {
//     let res = num1 + num2;
//     return res;
// }
// fn main() {
//     println!("{}", add(3, 2));
// }

//struct

// struct User {
//     active: bool,
//     username: String,
//     password: String,
//     email: String,
//     sign_in_count: u64,
//     age: i32,
// }

// fn main() {
//     let user1 = User {
//         active: true,
//         username: String::from("rustyn"),
//         password: String::from("jaiya1234@1728"),
//         email: String::from("jaiyankargupta@gmail.com"),
//         sign_in_count: 32,
//         age: 21,
//     };

//     println!(
//         "User1 username: {:?}, email: {:?}",
//         user1.username, user1.email
//     );
// }

// struct Rect {
//     width: u32,
//     height: u32,
// }

// impl Rect {
//     fn area(&self) -> u32 {
//         self.width * self.height
//     }
//     fn diameter(&self) -> u32 {
//         2 * self.width + 2 * self.height
//     }

//     fn debug() -> i32 {
//         //static function : no self parameter (we can use it without creating an instance of the struct)
//         return 1;
//     }
// }

// fn main() {
//     let rect1 = Rect {
//         width: 30,
//         height: 50,
//     };

//     println!(
//         "The area of the rectangle is {} square pixels.",
//         rect1.area(),
//     );
//     println!(
//         "The diameter of the rectangle is {} pixels.",
//         rect1.diameter()
//     );

//     println!("Debug: {}", Rect::debug()); //static function call
// }

//enum

// enum Shape {
//     Rectangle(f64, f64),
//     Square(f64),
// }

// fn calculate_area(shape: Shape) -> f64 {
//     let res = match shape {
//         Shape::Rectangle(a, b) => a * b,
//         Shape::Square(a) => 3.14 * a * a,
//     };
//     return res;
// }

// fn main() {
//     let value = Shape::Square(2.0);
//     let res = calculate_area(value);
//     println!("{}", res);
// }

//options

// fn main() {
//     let index = find_first_a(String::from("jaiyanakargupta"));

//     match index {
//         Some(value) => println!("{}", value),
//         None => println!("error"),
//     }
// }

// fn find_first_a(s: String) -> Option<i32> {
//     for (index, char) in s.chars().enumerate() {
//         if char == 'a' {
//             return Some(index as i32);
//         }
//     }
//     return None;
// }

//Result enum

// use std::fs;

// fn main() {
//     let out = fs::read_to_string("hello.txt");
//     match out {
//         Ok(content) => println!("{:?}", content),

//         Err(error) => println!("failed to read : {:?}", error),
//     }
// }

//use package

// use chrono::{Local, Utc};

// fn main() {
//     //get current date and time in UTC

//     let now = Utc::now();

//     println!("{}", now);

//     let formatted = now.format("%Y-%m-%d %H-%M-%S");
//     println!("{}", formatted);

//     //get local time

//     let localNow = Local::now();

//     println!("{}", localNow);

//     let formattedLocalNow = localNow.format("%Y-%m-%d %H-%M-%S");
//     println!("{}", formattedLocalNow);
// }

//ownership -> only one owner at a time.

// fn main() {
//     let a1 = String::from("jaiyankargupta");
//     //let a2 = a1 //move a1 to a2
//     let a2 = a1.clone();
//     println!("{}", a2);
//     println!("{}", a1);
// }

//vectors
// fn main() {
//     let mut vec = Vec::new();

//     //initialise the vector

//     // let mut number: Vec<i32> = vec![1, 2, 3];

//     vec.push(2);
//     vec.push(3);
//     println!("{:?}", even_vec(vec));
// }

// fn even_vec(vec: Vec<i32>) -> Vec<i32> {
//     let mut new_vec = Vec::new();
//     for val in vec {
//         if val % 2 == 0 {
//             new_vec.push(val);
//         }
//     }
//     return new_vec;
// }

//hashmap

use std::collections::HashMap;

fn get_group_value(vec: Vec<(String, i32)>) -> HashMap<String, i32> {
    let mut hm = HashMap::new();
    for (key, value) in vec {
        hm.insert(key, value);
    }
    return hm;
}

fn main() {
    let hm = vec![
        (String::from("jaiyankargupta"), 22),
        (String::from("rahul"), 31),
    ];
    let output = get_group_value(hm);
    println!("{:?}", output);
}
