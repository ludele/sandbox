use std::io::{self, Write};

fn print_stuff_add_numbers(x: i32, y: i32) -> i32 {
    println!(" x: {} and y: {}", x, y);

    if y < x {
        println!("y is smaller than x");
    } else {
        println!("x is smaller than y");
    }

    let result = x + y;
    println!("{} + {} = {}", x, y, result);
    result
}

fn input_number() -> i32 {
    print!("Enter a number: ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");

    // Attempt to parse the input into an i32
    match input.trim().parse::<i32>() {
        Ok(number) => number,
        Err(_) => {
            println!("Invalid input. Please enter a valid number.");
            return input_number();
        }
    }
}

fn main() {
    welcome_msg();
    let input_x = input_number();
    let input_y = input_number();
    print_stuff_add_numbers(input_x, input_y);
}

fn welcome_msg() {
    println!("Hello, World!");
    println!("Actually hello to you!");
}