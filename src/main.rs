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

struct Rect {
    width: u32,
    height: u32,
}

impl Rect {
    fn area(&self) -> u32 {
        self.width * self.height
    }
    fn diameter(&self) -> u32 {
        2 * self.width + 2 * self.height
    }
}

fn main() {
    let rect1 = Rect {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area(),
    );
    println!(
        "The diameter of the rectangle is {} pixels.",
        rect1.diameter()
    );
}
