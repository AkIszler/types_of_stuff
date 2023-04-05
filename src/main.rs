use core::num;
use std::io;


// just some practice on general functions and data type info.

fn main() {
  let mut y = {  //sets var for i32 "y"
        let x = 3; // gives x a value
        x + 1 // adds 1    
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


    let num = 3;

    if num != 3 {               // the number is NOT (!=) to 3
        println!("the number is 3!");
    }
    else if num == 2 {          // this wont trigger because the above statement is true
        println!("looks like its 2?")
    }
    else {                      // in case eveerything was true before, this will trigger
        println!("the number isnt 3");   
    }


    let num2 = 12;
    if num2 % 5 == 0 {                           // false so no trigger
         println!("the number is divisible by 4")
    }   
   else if num2 % 3 == 0 {                       // true so it will trigger
       println!("the number is divisible by 3")
   }
   else {                                       // wont trigger because the above statement is true
       println!("the number us {}", num2)
   } 

}

fn another_function(x: i32) {
    println!("The value of x is: {x}");
}

fn five() -> i32 {
    5
}

