use core::num;
use std::io;


// just some practice on general functions and data type info.

fn main() {

      let y = {
        let x = 3;
        x + 1
    };

    let z =five();
    println!("{z}");

    println!("The value of y is: {y}");

    println!("input a number");
    let mut number = String::new();

    io::stdin().read_line(&mut number)
    .expect("please use a number");
    let number: i32 = number.trim().parse().expect("please use a nunber");

    another_function(number);
}

fn another_function(x: i32) {
    println!("The value of x is: {x}");
}

fn five() -> i32 {
    5
}
