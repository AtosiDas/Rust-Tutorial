// Convert temperatures between Fahrenheit and Celsius.

use std::io;

fn main() {
    println!("Enter your choice");
    println!("1. Celcius to Fahrenhite");
    println!("2. Fahrenhite to Celcius");

    let mut choice = String::new();

    io::stdin() 
        .read_line(&mut choice)
        .expect("Failed to read line");
    let choice : u32 = choice.trim().parse().expect("Enter a number");
    println!("Your choice is {choice}");

    match choice {
        1 => {
            println!("Enter the Celcius value");
            let mut celcius = String::new();
            io::stdin()
                .read_line(&mut celcius)
                .expect("Failed to read the line");

            let celcius: f32 = celcius.trim().parse().expect("Failed to read the line");
            println!("The celcius value is: {celcius}");
            let mut fahrenheit =(( 9.0 * celcius) / 5.0) + 32.0;
            println!("The Fahrenhreit of {celcius}C is {fahrenheit}F");
        }
        2 => {
            println!("Enter the Fahrenhite value");
            let mut fahrenheit = String::new();
            io::stdin()
                .read_line(&mut fahrenheit)
                .expect("Failed to read the line");

            let fahrenheit: f32 = fahrenheit.trim().parse().expect("Failed to read the line");
            println!("The fahrenhite value is: {fahrenheit}");
            let mut celcius =( 5.0 * (fahrenheit - 32.0)) / 9.0;
            println!("The Celcius of {fahrenheit}F is {celcius}C");
        }
        _ => {
            println!("Enter a correct choice");
        }
    }
    
}