// Convert temperatures between Fahrenheit and Celsius.

use std::io;

fn main() {
    println!("Enter the celcius value");
    let mut celcius = String::new();
    io::stdin()
        .read_line(&mut celcius)
        .expect("Failed to read the line");

    let celcius: i32 = celcius.trim().parse().expect("Failed to read the line");
    println!("The celcius value is: {celcius}");
    let mut fahrenheit =(( 9 * celcius) / 5) + 32;
    println!("The Fahrenhreit of {celcius}C is {fahrenheit}F");
}