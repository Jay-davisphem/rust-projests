use std::{i32, io};

fn main() {
    let arr = [1, 2, 3, 4, 5];

    println!("Please enter an array index: ");

    let mut i = String::new();

    io::stdin().read_line(&mut i).expect("Failed to read line");

    let i: usize = i.trim().parse().expect("Index entered was not a number");

    let ele = arr[i];

    println!("The value of the element at index {i} is: {ele}");
    add(34, 3452);

    let cond = true;
    let number = if cond { 56 } else { 344 };

    loop {
        println!("The number is {number}");
    }
}

fn add(a: i128, b: i128) {
    println!("{:?}", a + b)
}
