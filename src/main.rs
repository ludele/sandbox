use std::io::{self, Write};

fn print_stuff_add_numbers(x: i32, y: i32) -> i32 {
    println!("\nx: {}, y: {}", x, y);

    if y < x {
        println!("y is smaller than x");
    } else if x < y {
        println!("x is smaller than y");
    } else {
        println!("same numbers");
    }

    let result = x + y;

    println!("{} + {} = {}", x, y, result);
    return result;

}

fn read_line() -> String {
    io::stdout().flush().expect("Failed to flush stdout");

    let mut input = String::new();

    io::stdin().read_line(&mut input).expect("Failed to read line");

    input.trim().to_string()
}

fn input_number() -> i32 {
    print!("Enter a number: ");

    let input = read_line();

    // Attempt to parse the input into an i32
    match input.trim().parse::<i32>() {
        Ok(number) => number,
        Err(_) => {
            println!("Invalid input. Please enter a valid number.");
            return input_number();
        }
    }
}

fn welcome_msg() {
    println!("Hello, World!");
    println!("Actually hello to you!");
}

fn run() {
    let input_x = input_number();
    let input_y = input_number();

    print_stuff_add_numbers(input_x, input_y);

    print!("\nDo you want to go again? yes/y?: ");

    let input: String = read_line();

    // println!("User Input: {:?}", input);

    match input.trim().to_lowercase().as_str() {
        "yes" | "y" => {
            run();
        } _ => {
            (); 
        }
    }
}
fn main() {
    welcome_msg(); 
    run();
}
