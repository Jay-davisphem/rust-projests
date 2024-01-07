const MAX_POINTS: u32 = 100_000;

fn main() {
    let x: &str = "Hello, world!";
    println!("\nThe value of MAX_POINTS is: {MAX_POINTS}");
    let x = x.len();
    println!("The value of x is: {x}");
    let x = " I am a Rustacean!";
    println!("The value of x is: {x}");

    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}\n");

    let mut binary = 0b1100_0000;
    println!("The value of binary is: {binary:0b}");

    binary <<= 2;
    println!("The value of binary is: {binary:0b}");

    let binary = binary >> 4;
    println!("The value of binary is: {binary:0b}");
}
