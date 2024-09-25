// fn main() {
//     println!("{}", is_even(2));
// }

// fn is_even(n: i32) -> bool {
//     if n % 2 == 0 {
//         return true;
//     } else {
//         return false;
//     }
// }

fn main() {
    println!("{}", fib(10));
}

fn fib(num: i32) -> i32 {
    let mut first: i32 = 0;
    let mut second: i32 = 1;
    if num == 0 {
        return num;
    } else if num == 1 {
        return num;
    }

    for _i in 1..(num - 2) {
        let temp = second;
        second = second + first;
        first = temp;
    }
    return second;
}

// fn main() {
//     let str = String::from("jaiyankargupta");
//     let res = find_len(str);
//     println!("{}", res);
// }

// fn find_len(str: String) -> usize {
//     str.chars().count()
// }
