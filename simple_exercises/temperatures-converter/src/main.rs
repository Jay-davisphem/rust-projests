// Path: simple_exercises/temperatures-converter/src/main.rs

use std::io;

fn main() {
    temperatures_display();

    loop {
        let result = get_result(enter_conversion(), get_temperature_value());

        println!("\nResult: {}", result);

        if !go_again() {
            break;
        }
        clear_screen();
    }
}

fn go_again() -> bool {
    println!("\nPress enter to continue or type 'exit' to quit");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    if input.trim().to_lowercase() == "exit" {
        false
    } else {
        true
    }
}
fn clear_screen() {
    // use std::process::Command;
    // Command::new("clear").status().unwrap();
    print!("\x1B[2J\x1B[1;1H");
}
fn temperatures_display() {
    let padding = "\t".repeat(3);
    println!("\n{padding} Temperatures Converter {padding}\n");
}

fn enter_conversion() -> String {
    println!("\nEnter temperature symbol, C(c) or F(c):");

    let mut sym = String::new();
    loop {
        io::stdin()
            .read_line(&mut sym)
            .expect("Failed to read line");

        sym = sym.trim().to_uppercase();

        if sym == "C" || sym == "F" {
            break;
        } else {
            println!("Please enter a valid symbol");
            sym.clear();
        }
    }

    sym
}

fn get_temperature_value() -> f64 {
    let mut temp = String::new();

    println!("\nEnter temperature value:");

    io::stdin()
        .read_line(&mut temp)
        .expect("Failed to read line");

    let temp: f64 = temp.trim().parse().expect("Value entered was not a number");
    temp
}

fn fahrenheit_to_celsius(temp: f64) -> f64 {
    (temp - 32.0) * 5.0 / 9.0
}

fn celsius_to_fahrenheit(temp: f64) -> f64 {
    (temp * 9.0 / 5.0) + 32.0
}

fn get_result(sym: String, temp: f64) -> String {
    if sym == "C" {
        format!("{:.2} °F", celsius_to_fahrenheit(temp))
    } else {
        format!("{:.2} °C", fahrenheit_to_celsius(temp))
    }
}
